use clap::{Parser, Subcommand};
use std::env;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Level of verbosity
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Backup {
        /// Location of dot-files (defaults to user home directory)
        #[arg(short = 's', long, required = true, value_hint = clap::ValueHint::DirPath)]
        source: String,

        /// Where to store backup vault
        #[arg(short = 'd', long, required = true)]
        destination: String,

        /// What storage backend is required
        #[arg(short = 't', long, required = true, value_names = ["file", "null"])]
        dest_type: String,
    },

    #[command(arg_required_else_help = true)]
    Restore {
        /// Location of your backup vault (i.e. the "destination" of the backup job)
        #[arg(short = 's', long, required = true)]
        source: String,

        /// What storage backend is required
        #[arg(short = 't', long, required = true, value_names = ["file"])]
        source_type: String,

        /// Where to restore the backup (i.e. the "source" of the backup job)
        #[arg(short = 'd', long, required = true)]
        destination: String,
    },
}

fn main() {
    // let app_config: AppConfig =
    //     dot_back::config_handler::load().expect("Unable to create application config");
    // println!("{:#?}", app_config);

    let cli = Args::parse();

    //  dot_back::backup::run(app_config);
}
