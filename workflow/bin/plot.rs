use clap::{ArgGroup, Args, Parser};
use std::path::PathBuf;
use workflow::conf::{
    get_tree_by_name,
    get_workflow_by_title,
    load_library,
    root_library_path,
    Library,
    // BehaviorTreeFile,
    // WorkflowFile,
    Node
};
use std::error::Error;

use log::{debug, trace, info};
use simplelog::*;

/// Different ways to input a workflow or tree file
#[derive(Args, Debug, Clone)]
#[clap(author, version, about, group = ArgGroup::new("input").multiple(false))]
#[group(required = true)]
struct InputFile {
    /// Path to the input tree
    #[clap(short = 'T', long, group = "input", conflicts_with_all = &["tree_name", "workflow_path", "workflow_name"])]
    tree_path: Option<PathBuf>,

    /// Path to the input tree
    #[clap(short = 'W', long, group = "input", conflicts_with_all = &["tree_name", "tree_path", "workflow_name"])]
    workflow_path: Option<PathBuf>,

    /// The name of the workflow to run
    #[clap(short = 'w', long, group = "input", conflicts_with_all = &["tree_name", "tree_path", "workflow_path"])]
    workflow_name: Option<String>,

    /// The name of the tree to run
    #[clap(short = 't', long, group = "input", conflicts_with_all = &["tree_path", "workflow_path", "workflow_name"])]
    tree_name: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "plotree", about = "Plot a workflow or tree", author, version, long_about = None)]
struct Cli {
    /// Path to the library folder
    #[clap(short, long)]
    library_path: Option<String>,

    /// Input file
    //#[args(short, long, flatten)]
    #[command(flatten)]
    input_file: InputFile,

    /// Output file path
    #[arg(short, long)]
    output_path: Option<PathBuf>
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the logger
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let args = Cli::parse();
    info!("{:#?}", &args);

    // Load the library
    let library_path = match args.library_path {
        Some(path) => PathBuf::new().join(path),
        None => root_library_path().expect("Unable to find library path"),
    };
    let library = load_library(&library_path).expect("Failed to load library");

    //let out = get_workflow_by_title(&args.workflow_name, &library).expect("Failed to get workflow");
    //trace!("{:?}", out);
    //let out = get_tree_by_name(&args.tree_name, &library).expect("Failed to get tree");
    //trace!("{:#?}", out);


    Ok(())
}
