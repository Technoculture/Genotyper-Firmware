use workflow::conf::{get_tree_by_name, get_workflow_by_title, home_library_path, load_library};
use clap::Parser;
use std::path::PathBuf;

use log::{info, trace};
use simplelog::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the library folder. 
    /// In a system following Linux FHS, this should be /var/tcr/library
    #[arg(short, long)]
    library_path: Option<String>,

    /// The name of the workflow to run
    #[arg(short, long, default_value = "TB PCR")]
    // workflow_name: Option<String>,
    workflow_name: String,

    /// The name of the tree to run
    #[arg(short, long, default_value = "attempt_pickup_tip")]
    // tree_name: Option<String>,
    tree_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the logger
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        // LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let args = Args::parse();
    info!("{:?}", &args);

    // Load the library
    let library_path = match args.library_path {
        Some(path) => PathBuf::new().join(path),
        None => home_library_path().expect("Unable to find library path")
    };
    let library = load_library(&library_path).expect("Failed to load library");

    let out = get_workflow_by_title(&args.workflow_name, &library).expect("Failed to get workflow");
    trace!("{:?}", out);
    let out = get_tree_by_name(&args.tree_name, &library).expect("Failed to get tree");
    trace!("{:#?}", out);

    Ok(())
}
