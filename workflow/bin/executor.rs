use clap::Parser;
use workflow::conf::{
    get_tree_by_name, load_library, root_library_path, BehaviorTreeFile, Library, Node,
};
use workflow::Sequence;

use log::{debug, info, trace};
use simplelog::*;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the workflow to run
    #[arg(short, long, default_value = "TB PCR")]
    // workflow_name: Option<String>,
    workflow_name: String,

    /// The name of the tree to run
    #[arg(short, long, default_value = "get_tip")]
    // tree_name: Option<String>,
    tree_name: String,
}

pub fn execute_node(node: &Node, library: &Library) -> Result<(), Box<dyn Error>> {
    // Execute the current node
    trace!("Executing node: '{}'", node.name);

    // TODO: Implement the actual node execution logic here
    // Recursively execute child nodes
    if let Some(nodes) = &node.sequence {
        match nodes {
            Sequence::Fallback(fallback_nodes) => {
                for node in fallback_nodes {
                    execute_node(node, library)?;
                }
            }
            Sequence::Children(children_nodes) => {
                for node in children_nodes {
                    execute_node(node, library)?;
                }
            }
        }
    } else {
        trace!("Executing leaf node: {}", node.name);
    }
    Ok(())
}

pub fn execute_tree(tree: &BehaviorTreeFile, library: &Library) -> Result<(), Box<dyn Error>> {
    info!(
        "Starting execution of a tree: '{}' ({})",
        tree.title, tree.description
    );

    // TODO: Traverse through the tree and execute the nodes
    // Traverse in a root-first manner
    execute_node(&tree.tree, library)?;

    Ok(())
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

    let args = Args::parse();
    debug!("{:?}", &args);

    // Load the library
    let library_path = root_library_path().expect("Unable to find library path");
    let library = load_library(&library_path).expect("Failed to load library");

    // let out = get_workflow_by_title(&args.workflow_name, &library).expect("Failed to get workflow");
    // trace!("{:?}", out);

    let tree = get_tree_by_name(&args.tree_name, &library).expect("Failed to get tree");
    execute_tree(&tree, &library)?;

    Ok(())
}
