use clap::Parser;
use std::path::PathBuf;
use workflow::conf::{get_tree_by_name, get_workflow_by_title, load_library, root_library_path};

use log::{debug, trace};
use simplelog::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the library folder
    #[arg(short, long)]
    library_path: Option<String>,

    /// The name of the workflow to run
    #[arg(short, long, default_value = "TB PCR")]
    // workflow_name: Option<String>,
    workflow_name: String,

    /// The name of the tree to run
    #[arg(short, long, default_value = "get_tip")]
    // tree_name: Option<String>,
    tree_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the logger
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let args = Args::parse();
    debug!("{:?}", &args);

    // Load the library
    let library_path = match args.library_path {
        Some(path) => PathBuf::new().join(path),
        None => root_library_path().expect("Unable to find library path"),
    };
    let library = load_library(&library_path).expect("Failed to load library");

    let out = get_workflow_by_title(&args.workflow_name, &library).expect("Failed to get workflow");
    trace!("{:?}", out);
    let out = get_tree_by_name(&args.tree_name, &library).expect("Failed to get tree");
    trace!("{:#?}", out);

    Ok(())
}
