use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use clap::{Parser, Subcommand};
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

#[derive(Parser)]
#[command(
    version,
    about = "fab: the f*cking awesome builder for programmatic CAD",
    long_about = "Computer-assisted 3D design for humans. Use fab to streamline your fabrication design process."
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    /// Create a new design file
    New {
        /// Name of the design
        name: String,

        /// Description saved to the design's metadata
        #[arg(long, short, default_value = "Describe your design here")]
        description: String,

        /// Directory for design files
        #[arg(long, short = 'D', default_value = "designs")]
        designs_dir: PathBuf,

        /// Overwrite an existing design
        #[arg(long, short)]
        overwrite: bool,

        /// Whether to also compile the design
        #[arg(long="compile", short='c')]
        also_compile: bool,
    },

    /// Compile and run a design, building it into something printable
    Compile {
        // Name of the design
        name: String,
    },
}

fn main() -> anyhow::Result<()> {

    match Cli::parse().command {

        Commands::New {
            name,
            description,
            designs_dir,
            overwrite,
            also_compile
        } => {

            if !designs_dir.is_dir() {
                fs::create_dir(&designs_dir)?;
            }

            let design_path = designs_dir.join(format!("{name}.rs"));

            if design_path.exists() && !overwrite {
                anyhow::bail!(
                    "Design already exists at {}, overwrite disabled.",
                    design_path.display()
                );
            }

            write_default_rs(&name, &description, &design_path)?;

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

            // if !cargo_toml["bin"].is_array_of_tables() {
            if !cargo_toml.contains_key("bin") {
                cargo_toml["bin"] = Item::ArrayOfTables(ArrayOfTables::new());
            }

            // Get a reference to an array of all the "bin" listings in Cargo.toml
            let binary_tables = cargo_toml["bin"]
                .as_array_of_tables_mut()
                .expect(
                    "Error getting array of binaries listed in Cargo.toml"
                );

            // If the design binary already exists, no need to edit, clean exit
            if binary_tables.iter().any(|bin| bin["name"].as_str() == Some(&name)) {
                return Ok(())
            }

            // Create a new binary entry for this design
            let mut my_binary_table = Table::new();
            my_binary_table["name"] = value(&name);
            my_binary_table["path"] =
                value(design_path.into_os_string().to_str().unwrap());

            // Push the new binary declaration
            binary_tables.push(my_binary_table);

            // Write updated Cargo.toml, closing Cargo.toml
            fs::write("Cargo.toml", cargo_toml.to_string())?;

            if also_compile {
                compile_design(&name)?;
            }
        }

        Commands::Compile { name } => compile_design(&name)?,
    }

    Ok(())
}

fn default_design_rs(name: &str, description: &str) -> String {
    format!(
r#"use monstertruck::modeling::Solid;

use fab::design::Design;
use fab::shapes::cube;

fn main() -> anyhow::Result<()> {{
    design().compile()
}}

pub fn design() -> Design {{
    Design::new({name:?}, {description:?})
        .set_spec(specification)
}}

pub fn specification() -> Solid {{
    // Put your code here to create custom designs
    cube(1.0).unwrap()
}}

"#
    )
}

fn write_default_rs(
    name: &str,
    description: &str,
    design_path: &Path,
) -> anyhow::Result<()> {

    fs::write(
        design_path,
        default_design_rs(name, description)
    )?;

    Ok(())
}

fn compile_design(name: &str) -> anyhow::Result<()> {

    // Without this file, cargo can't find the path to the binary
    anyhow::ensure!(
        Path::new("Cargo.toml").is_file(),
        "No Cargo.toml found in the current directory"
    );

    // Run the compiled binary using cargo, which rebuilds it if necessary
    anyhow::ensure!(

        Command::new("cargo")
            .args(["run", "--bin", name])
            .status()?
            .success(),

        "Rust file defining the design `{name}` failed to compile or run."
    );

    Ok(())
}

