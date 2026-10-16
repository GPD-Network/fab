use clap::{Parser, Subcommand};
use console::Term;


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

        /// Name of the design; user will be prompted for it if not given
        #[arg(long, short)]
        name: Option<String>,

        /// Description for the design saved to design's metadata
        #[arg(long, short)]
        description: String,

        /// Directory to use for designs, defaults to `designs` in working directory
        #[arg(long, short='D')]
        designs_dir: String,

        /// Pair of shape:arg1,arg2,arg3 pairs; e.g. cylinder:(0.0,0.0,0.0),5,10 for 10mm tall cylinder with 10mm diameter base
        #[arg(long, short)]
        predrawn: Vec<String>,

        /// Whether to overwrite existing design
        #[arg(long, short='o')]
        overwrite: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::New { name, description, designs_dir, predrawn, overwrite } => {

            // Connect to stdout for communicating with user
            let term = Term::stdout();

            // If user provided predrawn arguments, tell them not yet used
            if !predrawn.is_empty() {
                term.write_line("Predrawn designs requested but this feature is not yet implemented. Sorry!")
                    .expect("Failure writing to terminal...");
            }

            //
        }
    }
}


fn design_topmatter(name: String, description: String) -> anyhow::Result<String> {

    let ret = "yo";
    let ret =
"
gT
"

    Ok(ret.to_string())
}
