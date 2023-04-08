use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Library {
    modules: HashMap<String, Module>,
    tools: HashMap<String, Tool>,
    nodes: HashMap<String, KnownNode>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Module {
    name: String,
    #[serde(rename = "type")]
    module_type: String,
    address: String,
    description: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    //#[serde(default)]
    pick_up: Option<String>,
    //#[serde(default)]
    variants: Option<Vec<Variant>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Variant {
    name: String,
    description: Option<String>,
    preffered_when: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum NodeType {
    Condition,
    Action,
    Sequence,
    Error,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct KnownNode {
    #[serde(rename = "type")]
    node_type: NodeType,
    #[serde(default)]
    zenoh: Option<Zenoh>,
    description: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ReplyMode {
    FromAny,
    FromAll,
    FromOne,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Zenoh {
    modules: Vec<String>,
    min_reply: ReplyMode,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Node {
    name: String,
    error: Option<String>,
    children: Option<Vec<Node>>,
    fallback: Option<Vec<Node>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BehaviorTreeFile {
    title: String,
    version: String,
    description: String,
    #[serde(rename = "participant_modules")]
    participants: Vec<String>,
    tree: Node,
}

//fn serialize_behaviour_tree(tree: &BehaviorTreeFile, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
//    let yaml_string = serde_yaml::to_string(&tree).expect("Failed to serialize tree");
//    let mut file = File::create(file_name).expect("Failed to create file");
//    file.write_all(yaml_string.as_bytes()).expect("Failed to write to file");
//    Ok(())
//}

fn load_a_behaviour_tree(file_name: &str) -> Result<BehaviorTreeFile, Box<dyn std::error::Error>> {
    let file = File::open(file_name).expect("Failed to open file");
    let tree: BehaviorTreeFile = serde_yaml::from_reader(file).expect("Failed to deserialize tree");
    println!("{:#?}", tree);
    Ok(tree)
}

//fn extract_names(node: &Node, names: &mut Vec<String>) {
//    names.push(node.name.clone());
//    if let Some(children) = &node.children {
//        for child in children {
//            extract_names(child, names);
//        }
//    }
//}

fn load_file_modules(path: &str) -> Result<HashMap<String, Module>, Box<dyn std::error::Error>> {
    // Load the modules
    let modules_file =
        File::open(&path)
            .expect(format!("Unable to open file {}", &path).as_str());
    let modules: HashMap<String, Module> =
        serde_yaml::from_reader(modules_file).expect("Unable to parse modules");
    println!("Modules file is valid and parsed correctly");
    Ok(modules)
}

fn load_file_tools(path: &str) -> Result<HashMap<String, Tool>, Box<dyn std::error::Error>> {
    // Load the tools
    let tools_file = File::open(path).expect("Unable to open file");
    let tools: HashMap<String, Tool> =
        serde_yaml::from_reader(tools_file).expect("Unable to parse tools");
    println!("Tools file is valid and parsed correctly");
    Ok(tools)
}

fn load_file_nodes(
    path: &str,
) -> Result<HashMap<String, KnownNode>, Box<dyn std::error::Error>> {
    // Load the nodes
    let nodes_file = File::open(path).expect("Unable to open file");
    let nodes: HashMap<String, KnownNode> =
        serde_yaml::from_reader(nodes_file).expect("Unable to parse nodes");
    println!("Nodes file is valid and parsed correctly");
    Ok(nodes)
}

fn dependencies_abbr(
    modules: &HashMap<String, Module>,
    tools: &HashMap<String, Tool>,
) -> Vec<String> {
    // modules and tools are the same, so we need to collect both their abbr in a single vector
    let mut abbrs = Vec::new();
    for (name, _) in modules {
        abbrs.push(name.clone());
    }
    for (name, _) in tools {
        abbrs.push(name.clone());
    }
    println!("Modules and Tools in Library: {:?}", &abbrs);
    abbrs
}

fn validate_nodes_library(
    nodes: &HashMap<String, KnownNode>,
    known_dependencies: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Throw an error if a node is using a module that is not known
    for (name, node) in nodes {
        if let Some(zenoh) = &node.zenoh {
            for module in &zenoh.modules {
                if !known_dependencies.contains(module) {
                    return Err(
                        format!("Node {} is using an unknown module {}", name, module).into(),
                    );
                }
            }
        }
    }
    println!("KnownNode Library is valid and ready to be used");
    Ok(())
}

fn load_library(library_path: &PathBuf) 
    -> Result<Library, Box<dyn std::error::Error>> 
{
    let modules = load_file_modules(library_path.join("modules.yaml").to_str().unwrap()).expect("Failed to load modules");
    let tools = load_file_tools(library_path.join("tools.yaml").to_str().unwrap()).expect("Failed to load tools");
    let known_dependencies = dependencies_abbr(&modules, &tools);

    let nodes = load_file_nodes(library_path.join("nodes.yaml").to_str().unwrap()).expect("Failed to load nodes");
    validate_nodes_library(&nodes, &known_dependencies).expect("Failed to validate nodes library");
    Ok(Library {
        modules,
        tools,
        nodes,
    })
}

fn validate_btree(
    tree_file: &BehaviorTreeFile,
    library: &Library,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if btree name is not a known node
    if library.nodes.contains_key(&tree_file.tree.name) {
        return Err(format!(
            "Behavior Tree name {} is already used by a known node",
            tree_file.tree.name
        )
        .into());
    }

    // Check if participants are known
    for participant in &tree_file.participants {
        if !library.modules.contains_key(participant) {
            return Err(format!("Participant Module {} is not a known node", participant).into());
        }
    }

    validate_node(&tree_file.tree, &library).expect("Failed to validate node");

    println!("Behavior Tree file {} is valid and ready to be used", tree_file.tree.name);

    Ok(())
}

fn validate_node(tree: &Node, library: &Library) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the tree is valid
    // 1. If the node has children, check if they are valid
    //  a. If a node has no children, check if it is a known node
    //  b. If all children are valid, the node is valid
    // 2. If node has fallback, check if all fallback nodes are valid
    // 3. If all nodes are valid, the tree is valid

    if let Some(fallback) = &tree.fallback {
        // If the node has fallback, check if all fallback nodes are valid
        for child in fallback {
            //  If all nodes are valid, the tree is valid
            validate_node(child, library)?;
        }
    } else if let Some(children) = &tree.children {
        // If the node has children, check if they are valid
        for child in children {
            //  If all children are valid, the node is valid
            validate_node(child, library)?;
        }
    } else {
        // If a node has no children, check if it is a known node
        if !library.nodes.contains_key(&tree.name) {
            return Err(format!("Node {} is not a known node", tree.name).into());
        }
    }

    Ok(())
}

fn library_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut home_dir = dirs::home_dir().expect("Unable to find home directory");
    home_dir.push("genotyper");
    home_dir.push("library");
    Ok(home_dir)
}

fn main() -> Result<(), serde_yaml::Error> {
    // Load the library
    let library_path = library_path().expect("Unable to find library path");
    let library = load_library(&library_path).expect("Failed to load library");

    // Load the btree
    let mut btree_file_path = library_path.clone();
    btree_file_path.push("trees");
    btree_file_path.push("attempt_pickup_tip-0.0.1.yaml");
    let btree_file = load_a_behaviour_tree(btree_file_path.to_str().unwrap()).expect("Failed to deserialize tree");
    validate_btree(&btree_file, &library).expect("Failed to validate tree");

    //let mut names = Vec::new();
    //extract_names(&btree_file.tree, &mut names);
    //println!("Nodes in Tree File: {:?}", names);
    println!("btree.yaml is valid and parsed correctly");

    Ok(())
}
