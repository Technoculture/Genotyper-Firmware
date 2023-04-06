use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::Write;
//use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Library {
    modules: Vec<Module>,
    tools: Vec<Tool>,
    nodes: Vec<KnownNode>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Module {
    name: String,
    #[serde(rename = "type")]
    module_type: String,
    address: String,
    abbr: String,
    description: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    //#[serde(default)]
    pick_up: Option<String>,
    abbr: String,
    //#[serde(default)]
    variants: Option<Vec<Variant>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Variant {
    name: String,
    description: Option<String>,
    abbr: String,
    preffered_when: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum NodeType {
    #[serde(rename = "condition")]
    Condition,
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "sequence")]
    Sequence,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct KnownNode {
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    //#[serde(default)]
    zenoh: Option<Zenoh>,
    description: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ReplyMode {
    #[serde(rename = "from_any")]
    FromAny,
    #[serde(rename = "from_all")]
    FromAll,
    #[serde(rename = "from_one")]
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
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BehaviorTree {
    tree: Vec<Node>
}

fn serialize_behaviour_tree(tree: &BehaviorTree, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_string = serde_yaml::to_string(&tree).expect("Failed to serialize tree");
    let mut file = File::create(file_name).expect("Failed to create file");
    file.write_all(yaml_string.as_bytes()).expect("Failed to write to file");
    Ok(())
}

fn deserialize_behaviour_tree(file_name: &str) -> Result<BehaviorTree, Box<dyn std::error::Error>> {
    let file = File::open(file_name).expect("Failed to open file");
    let tree: BehaviorTree = serde_yaml::from_reader(file).expect("Failed to deserialize tree");
    Ok(tree)
}

fn main() -> Result<(), serde_yaml::Error> {
    // Read all YAML files
    // ----
    //
    let library_path = "library/";
    let nodes_file_name = "nodes.yaml";
    let modules_file_name = "modules.yaml";
    let tools_file_name = "tools.yaml";

    // Load the modules
    let modules_file =
        File::open(library_path.to_owned() + modules_file_name).expect("Unable to open file");
    let modules: Vec<Module> =
        serde_yaml::from_reader(modules_file).expect("Unable to parse modules");
    //println!("Modules: {:#?}", modules);
    println!("Modules file is valid and parsed correctly");

    // Load the tools
    let tools_file =
        File::open(library_path.to_owned() + tools_file_name).expect("Unable to open file");
    let tools: Vec<Tool> = serde_yaml::from_reader(tools_file).expect("Unable to parse tools");
    //println!("Tools: {:#?}", tools);
    println!("Tools file is valid and parsed correctly");

    // modules and tools are the same, so we need to collect both their abbr in a single vector
    let modules_abbrs: Vec<String> = modules
        .iter()
        .map(|item| item.abbr.clone())
        .collect();
    let tool_abbrs: Vec<String> = tools
        .iter()
        .map(|item| item.abbr.clone())
        .collect();
    let known_dependencies = [&modules_abbrs[..], &tool_abbrs[..]].concat();

    println!("Modules and Tools in Library: {:?}", &known_dependencies);

    // Load the nodes
    let nodes_file =
        File::open(library_path.to_owned() + nodes_file_name).expect("Unable to open file");
    let nodes: Vec<KnownNode> = serde_yaml::from_reader(nodes_file).expect("Unable to parse nodes");
    //println!("{:#?}", nodes);

    // Throw an error if a node is using a module that is not known
    nodes.iter().for_each(|n| {
        if let Some(z) = &n.zenoh {
            z.modules.iter().for_each(|m| {
                if !known_dependencies.contains(m) {
                    panic!("KnownNode {} is using an unknown module {}", n.name, m);
                }
            });
        }
    });

    println!("KnownNode Library is valid and ready to be used");

    // ----
    //
    let btree = deserialize_behaviour_tree("btree.yaml").expect("Failed to deserialize tree");
    println!("{:#?}", btree);
    println!("btree.yaml is valid and parsed correctly");

    Ok(())
}
