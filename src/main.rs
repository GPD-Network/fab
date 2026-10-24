use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail,Context,ensure};
use anyhow::Result as AResult;
use clap::{Parser, Subcommand};
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};


const CARGO_PATH_STR: &str = "Cargo.toml";

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
        /// Name of the design; must already exist via fab new {name}
        name: String,
    },
}

fn main() -> AResult<()> {

    match Cli::parse().command {

        // === NEW
        Commands::New {
            name,
            description,
            designs_dir,
            overwrite,
            also_compile
        } => {

            // Create the directory passed by the user if necessary
            if !designs_dir.is_dir() {
                fs::create_dir(&designs_dir)?;
            }

            // Build path to the design
            let design_path = designs_dir.join(format!("{name}.rs"));

            // Bail if design already exists and user didn't indicate overwrite
            if design_path.exists() && !overwrite {
                bail!(
                    "Design already exists at {}, overwrite disabled.",
                    design_path.display()
                );
            }

            // Ensure the project's Cargo.toml is configured for fab
            setup_cargo_toml(&name, &design_path)?;

            // Create a design stub
            write_default_rs(&name, &description, &design_path)?;

            // Compile the design if user desires
            if also_compile {
                compile_design(&name)?;
            }
        }

        Commands::Compile { name } => compile_design(&name)?,
    }

    Ok(())
}

// Produce a string that defines a stub design for `fab new`
fn default_design_rs(name: &str, description: &str) -> String {

    format!(
r#"use anyhow::Result as AResult;

use monstertruck::modeling::Solid;

use fab::design::Design;
use fab::shapes::cube;

fn main() -> AResult<()> {{
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
) -> AResult<()> {

    fs::write(
        design_path,
        default_design_rs(name, description)
    )?;

    Ok(())
}

fn compile_design(name: &str) -> AResult<()> {

    // Without this file, cargo can't find the path to the binary
    ensure!(
        Path::new(CARGO_PATH_STR).is_file(),
        "No Cargo.toml found in the current directory"
    );

    // Run the compiled binary using cargo, which rebuilds it if necessary
    ensure!(

        Command::new("cargo")
            .args(["run", "--bin", name])
            .status()?
            .success(),

        "Rust file defining the design `{name}` failed to compile or run."
    );

    Ok(())
}

fn sync_fab_cargo_dependencies() -> AResult<()> {

    let path = CARGO_PATH_STR;
    let content = fs::read_to_string(path)?;
    let mut doc = content.parse::<DocumentMut>()?;

    let required =
r#"[dependencies]
anyhow = "1.0.104"
chrono = { version = "0.4.45", features = ["clock"] }
clap = { version = "4.6.7", features = ["derive"] }
console = "0.16.6"
dateparser = "0.3.1"
directories = "6.0.0"
fab = { git = "https://github.com/gpd-network/fab", version = "0.1.0" }
js-sys = "=0.3.85"
monstertruck = { version = "0.4.1", features = ["full"] }
toml_edit = "0.25.17"
"#;

    let required = required.parse::<DocumentMut>()?;

    let dependencies_iter = required["dependencies"]
        .as_table()
        .unwrap()
        .iter();
    for (name, dependency) in dependencies_iter {
        if doc["dependencies"].get(name).is_none() {
            doc["dependencies"][name] = dependency.clone();
        }
    }

    fs::write(path, doc.to_string())?;

    Ok(())
}

fn setup_cargo_toml(name: &str, design_path: &Path) -> AResult<()> {

    // Load or create a new Cargo.toml.
    let mut cargo_file = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(CARGO_PATH_STR)
        .context(format!("Error reading {CARGO_PATH_STR}"))?;

    let mut cargo_text: String = cargo_file
        .read_to_string(&mut String::new())?
        .to_string();

    if !cargo_text.contains("package") {

        // Use the user's current directory name as the cargo project name
        let project_dir  = std::env::current_dir()?;

        let project_name = project_dir
            .file_name()
            .and_then(|name| name.to_str());

        let mut new_cargo_text = format!(
r#"
[package]
name = {:?}
version = "0.1.0"
edition = "2024"

"#,
            project_name.unwrap()
        );

        if !cargo_text.contains("dependencies") {
            new_cargo_text.push_str(&format!("\n[dependencies]"))
        }

        cargo_text = new_cargo_text
    }

    // Convert text of cargo file to a TOML document
    println!("{cargo_text}");
    let mut cargo_toml = cargo_text
        .parse::<DocumentMut>()
        .context("Could not parse Cargo.toml")?;

    // Compilation steps vary depending on presence of binaries
    let cargo_has_binaries: bool = cargo_toml.contains_key("bin");

    // Check if any bin entries exist, creating the first one if not
    if !cargo_has_binaries {
        cargo_toml["bin"] = Item::ArrayOfTables(ArrayOfTables::new());
    }

    // Get all binaries as an array of all the "bin" listings in Cargo.toml
    let binary_tables = cargo_toml["bin"]
        .as_array_of_tables_mut()
        .expect(
            "Error getting array of binaries listed in Cargo.toml"
        );

    // Check if the binary already exists with a closure for lazy eval
    let binary_exists = || {
        binary_tables
            .iter()
            .any(|bin| bin["name"].as_str() == Some(name))
    };

    // If the design binary already exists, no need to add, exit clean
    if cargo_has_binaries && binary_exists() {
        return Ok(())
    }

    // Create a new binary entry for this design
    let mut my_binary_table = Table::new();
    my_binary_table["name"] = value(name);
    my_binary_table["path"] = value(design_path.to_str().unwrap());

    // Push the new binary declaration
    binary_tables.push(my_binary_table);

    // Write updated Cargo.toml, closing Cargo.toml
    fs::write(CARGO_PATH_STR, cargo_toml.to_string())?;

    // Close by appending fab's dependencies to Cargo.toml if necessary
    sync_fab_cargo_dependencies()
}
