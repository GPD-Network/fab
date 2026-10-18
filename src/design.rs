/// Data structure for automated creation and tracking of fab designs
use std::{
    fmt,
    fs,
    path::Path
};

use anyhow::Context;
use chrono::{DateTime,Local};

use monstertruck::{

    modeling::Solid,

    step::save::{
        CompleteStepDisplay,
        StepHeaderDescriptor,
        StepModel
    }
};

use toml_edit::{value, ArrayOfTables, DocumentMut, Item, Table};

use crate::shapes::cube;


fn default_cube_spec() -> Solid {

    // Just unwrapping bc we don't expect problems with a simple shape
    // like a cube. More complicated shapes may encounter unexpected problems.
    // Use standard, robust, idiomatic error handling instead of this.
    cube(1.0).unwrap()
}

pub struct Design {
    name: String,
    description: String,
    date: DateTime<Local>,
    spec: fn() -> Solid
}

impl Design {

    /// Create a new design with the given name
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>
    )
    -> Design {

        Self {
            name: name.into(),
            description: description.into(),
            date: Local::now(),
            spec: default_cube_spec
        }
    }

    pub fn set_spec(mut self, spec: fn() -> Solid)  -> Design {
        self.spec = spec;
        self
    }

    /// We compile a design by building and running the rest-based design
    pub fn compile(self, write_path: &Path) -> anyhow::Result<()> {

        // Flatten the topology and render as STEP.
        let compressed = (self.spec)().compress();

        std::fs::write(

            write_path,

            CompleteStepDisplay::new(

                StepModel::from(&compressed),

                StepHeaderDescriptor {
                    organization_system: "monstertruck-doc".to_owned(),
                    ..Default::default()
                },

            ).to_string()
        )?;

        println!("Wrote design {} to step file {}.", self.name, write_path.display());

        Ok(())
    }

    pub fn sync_metadata(self) -> anyhow::Result<()> {

        let cargo_text = fs::read_to_string("Cargo.toml")
            .context(format!(
r#"
Could not read Cargo.toml; run fab from a Cargo project root.\

Hint: Initialize new cargo project using `cargo new my-design-project`.
"#
))?;

        let mut cargo_toml = cargo_text
            .parse::<DocumentMut>()
            .context("could not parse Cargo.toml")?;

        if !cargo_toml["bin"].is_array_of_tables() {
            cargo_toml["bin"] = Item::ArrayOfTables(ArrayOfTables::new());
        }

        // Get a reference to an array of all the "bin" listings in Cargo.toml
        let binary_tables = cargo_toml["bin"]
            .as_array_of_tables_mut()
            .expect(
                "Error getting array of binaries listed in Cargo.toml"
            );

        // If the design binary already exists, no need to edit, clean exit
        if binary_tables.iter().any(|bin| bin["name"].as_str() == Some(&self.name)) {
            return Ok(())
        }

        // Create a new binary entry for this design
        let mut my_binary_table = Table::new();
        my_binary_table["name"] = value(&self.name);
        my_binary_table["path"] = value(file!());

        // Push the new binary declaration
        binary_tables.push(my_binary_table);

        // Now read the manifest if it exists, creating it if not
        let mut manifest_toml =
            if Path::new("Manifest.toml").try_exists()? {

                fs::read_to_string("Manifest.toml")?
                    .parse::<DocumentMut>()
                    .context("could not parse Manifest.toml")?
            } else {

                DocumentMut::new()
        };

        // Create a new design entry in the auto-generated manifest
        let design_entry = manifest_toml.entry(&self.name).or_insert(toml_edit::table());
        design_entry["description"] = value(self.description);
        design_entry["date"] = value(self.date.format("%Y-%m-%d").to_string());

        fs::write("Cargo.toml", cargo_toml.to_string())?;
        fs::write("Manifest.toml", manifest_toml.to_string())?;

        Ok(())
    }
}

impl fmt::Display for Design {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(
            f,
            "=== {}\nLocation: {}\n\nDescription:\n{}\n\n",
            self.name, self.description, file!()
        )
    }
}

impl fmt::Debug for Design {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let loc = file!();
        println!("Debug for design at {loc}");

        f.debug_struct("Design")
            .field("name", &self.name)
            .field("date", &self.date)
            .finish()
    }
}

pub fn load_design(design_path: &Path) -> anyhow::Result<Design> {

    todo!()
}
