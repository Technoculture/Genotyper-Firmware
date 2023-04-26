use clap::Parser;
use workflow::conf::{
    get_tree_by_name, load_library, root_library_path, BehaviorTreeFile, Library, ModuleFile, Node,
};
use workflow::Sequence;
use log::{debug, info, trace};
use simplelog::*;
use std::error::Error;

use async_std::prelude::*;
use async_std::channel::{bounded, Receiver, Sender};
use async_std::task;
use std::io::{stdin, BufRead};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use async_recursion::async_recursion;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the workflow to run
    #[arg(short, long, default_value = "tb_pcr")]
    // workflow_name: Option<String>,
    workflow_name: String,

    /// The name of the tree to run
    #[arg(short, long, default_value = "get_tip")]
    // tree_name: Option<String>,
    tree_name: String,
}

async fn module_task(receiver: Receiver<Node>, modules: &ModuleFile) {
    while let Ok(node) = receiver.recv().await {
        execute_leaf_node(&node, modules).await;
    }
}

async fn execute_leaf_node(node: &Node, modules: &ModuleFile) -> Result<(), Box<dyn Error>> {
    info!("Executing leaf node: {}", node.name);
    async_std::task::sleep(std::time::Duration::from_millis(200)).await;
    Ok(())
}

#[async_recursion]
pub async fn execute_node(node: &Node, library: &Library) -> Result<(), Box<dyn Error>> {
    trace!("Executing node: '{}'", node.name);
    // TODO: Implement the actual node execution logic here
    // Recursively execute child nodes
    if let Some(nodes) = &node.sequence {
        match nodes {
            Sequence::Fallback(fallback_nodes) => {
                for node in fallback_nodes {
                    execute_node(node, library).await?;
                }
            }
            Sequence::Children(children_nodes) => {
                for node in children_nodes {
                    execute_node(node, library).await?;
                }
            }
        }
    } else {
        trace!("Executing leaf node: {}", node.name);
        // async_std::task::block_on(execute_leaf_node(node, &library.modules))?;
        execute_leaf_node(node, &library.modules).await?;
    }
    Ok(())
}

async fn execute_tree(tree: &BehaviorTreeFile, library: &Library) -> Result<(), Box<dyn Error>> {
    info!(
        "Starting execution of a tree: '{}' ({})",
        tree.title, tree.description
    );
    execute_node(&tree.tree, library).await
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    execute_tree(&tree, &library).await
}
