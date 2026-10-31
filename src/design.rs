//! Automated creation and tracking of fab designs
//!
//! For command-line interface documentation please see
//! sustainableinference.com/fab/cli
use std::{
    fmt,
    fs,
    path::Path
};

use anyhow::Context;
use anyhow::Result as AResult;
use chrono::{DateTime,Local};

use monstertruck::step::save::{
        CompleteStepDisplay,
        StepHeaderDescriptor,
        StepModel
    };

use toml_edit::{value, DocumentMut};

use crate::shapes::Shape3D;


const BUILD_DIR: &str = "build";


#[derive(Clone)]
pub struct Design<T: Shape3D> {
    pub name: String,
    pub description: String,
    pub date: DateTime<Local>,
    pub spec: fn () -> T
}

impl<T: Shape3D> Design<T> {

    /// Create a new design with the given name
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        spec:  fn() -> T,
    ) -> Design<T> {


        Self {
            name: name.into(),
            description: description.into(),
            date: Local::now(),
            spec
        }
    }

    pub fn set_spec(mut self, spec: fn() -> T)  -> Design<T> {
        self.spec = spec;
        self
    }

    /// We compile a design by building and running the rust-based design
    pub fn compile(&self) -> AResult<()> {

        // Flatten the topology and render as STEP.
        let spec_shape = (self.spec)();
        let spec_model = spec_shape.model();

        let build_dir = Path::new(BUILD_DIR);

        if !build_dir.is_dir() {
            fs::create_dir(build_dir)?;
        }

        let write_path = build_dir.join(format!("{}.step", self.name));

        std::fs::write(

            &write_path,

            CompleteStepDisplay::new(

                StepModel::from(&spec_model.compress()),

                StepHeaderDescriptor {
                    organization_system: "monstertruck-doc".to_owned(),
                    ..Default::default()
                },
            ).to_string()
        ).context("Error writing the step file.")?;

        println!("Wrote design {} to step file {}.", self.name, write_path.display());

        Ok(())
    }

    pub fn sync_metadata(self) -> AResult<()> {

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
        let design_entry = manifest_toml
            .entry(&self.name)
            .or_insert(toml_edit::table());

        design_entry["description"] = value(self.description);

        design_entry["date"] = value(self.date.format("%Y-%m-%d").to_string());

        fs::write("Manifest.toml", manifest_toml.to_string())?;

        Ok(())
    }
}

impl<T: Shape3D> fmt::Display for Design<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(
            f,
            "=== {}\nLocation: {}\n\nDescription:\n{}\n\n",
            self.name, self.description, file!()
        )
    }
}

impl<T: Shape3D> fmt::Debug for Design<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        f.debug_struct("Design")
            .field("name", &self.name)
            .field("date", &self.date)
            .finish()
    }
}

