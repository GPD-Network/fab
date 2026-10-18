use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use clap::{Parser, Subcommand};

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

    /// Compile and run a design
    Compile {
        /// Name of the design
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
        r#"\
use fab::design::{{Design, default_cube_spec}};

pub fn design() -> Design {{
    Design::new({name:?}, {description:?})
        .set_spec(default_cube_spec)
}}

fn main() -> anyhow::Result<()> {{
    design().sync_printable()
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

        "Design `{name}` failed to compile or run"
    );

    Ok(())
}

