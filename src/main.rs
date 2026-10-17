use std::path::Path;
use std::fs;

use clap::{Parser, Subcommand};
// use console::Term;


#[derive(Parser)]
#[command(
    version,
    about = "fab: the f*cking awesome builder",
    long_about = "Computer-assisted 3D design for humans. Use fab to streamline your fabrication design process."
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    New {
        /// Name of the design, required
        name: String,

        /// Description for the design saved to design's metadata
        #[arg(long, short, default_value="Describe your design here")]
        description: String,

        /// Directory to use for designs, created if it does not exist
        #[arg(long, short='D', default_value="designs")]
        designs_dir: String,

        /// Whether to overwrite existing design
        #[arg(long, short='o')]
        overwrite: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::New { name, description, designs_dir, overwrite } => {


            let designs_dir = Path::new(designs_dir);
            let design_path = designs_dir.join("{name}.rs");

            // Create designs_dir if it does not yet exist and initialize design
            if !designs_dir.is_dir() {
                // Create directory
                fs::create_dir(designs_dir)?;
                // Write starter design to design_path
                fs::write(design_path, default_design_rs(name, description))?;
            } else {
                // If file exists and overwrite is false, exit
                if design_path.exists() && !overwrite {
                    anyhow::bail!(
                        "Design already exists at {}, overwrite disabled.",
                        design_path.display()
                    )
                }

                write_default_rs(name, description, design_path)
            }


        }
    }

    Ok(())
}


fn default_design_rs(name: &str, description: &str) -> String {

    format!(
        "\
        /* design {name} description: {description}
        "
    )
}

fn write_default_rs(name, description, design_path) -> anyhow::Result<()> {
    fs::write(design_path, default_design_rs(name, description))?;

    Ok(())
}

