use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use serde_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Library {
    modules: Vec<Module>,
    tools: Vec<Tool>,
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Module {
    name: String,
    #[serde(rename = "type")]
    module_type: String,
    address: String,
    annotation: Annotation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Tool {
    name: String,
    annotation: Annotation,
    //#[serde(default)]
    pick_up: Option<String>,
    //#[serde(default)]
    variants: Option<Vec<Variant>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Variant {
    name: String,
    annotation: Annotation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Node {
    name: String,
    #[serde(rename = "type")]
    node_type: String,
    //#[serde(default)]
    zenoh: Option<Zenoh>,
    annotation: Annotation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Zenoh {
    modules: Vec<String>,
    min_reply: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Annotation {
    abbr: Option<String>,
    description: Option<String>,
    preferred_when: Option<String>,
    need_module: Option<String>,
    needs_tool: Option<String>,
}

fn main() -> Result<(), serde_yaml::Error> {
    let library_file_name = "library.yaml";
    let mut library_file = File::open(library_file_name).expect("Unable to open library file");
    let mut library_file_content = String::new();
    library_file.read_to_string(&mut library_file_content).expect("Unable to read library file");
    let library: Library = serde_yaml::from_str(&library_file_content).expect("Unable to parse library file");

    println!("{:#?}", library);

    Ok(())
}
