use log::{debug, info, trace};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

const DONE: &str = "→ ✅";
const OK: &str = "↓ ✔️";

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    modules: ModuleFile,
    tools: ToolFile,
    nodes: KnownNodesFile,
    trees: Vec<BehaviorTreeFile>,
    workflows: Vec<WorkflowFile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModuleFile {
    version: String,
    endpoint: String,
    content: HashMap<String, Module>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Module {
    info: ModuleInfo,
    // TODO: add api
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ModuleInfo {
    name: String,
    #[serde(rename = "type")]
    module_type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ToolFile {
    version: String,
    content: HashMap<String, Tool>,
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
struct KnownNodesFile {
    version: String,
    content: HashMap<String, KnownNode>,
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
    Any,
    All,
    One,
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
pub struct BehaviorTreeFile {
    title: String,
    version: String,
    description: String,
    #[serde(rename = "participant_modules")]
    participants: Vec<String>,
    tree: Node,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowFile {
    title: String,
    description: String,
    version: String,
    workflow: Vec<WorkflowStep>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WorkflowStep {
    name: String,
    why: String,
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
    trace!("{:#?}", tree);
    Ok(tree)
}

fn load_a_workflow(file_name: &str) -> Result<WorkflowFile, Box<dyn std::error::Error>> {
    let file = File::open(file_name).expect("Failed to open file");
    let workflow: WorkflowFile = serde_yaml::from_reader(file).expect("Failed to deserialize tree");
    trace!("{:#?}", workflow);
    Ok(workflow)
}

//fn extract_names(node: &Node, names: &mut Vec<String>) {
//    names.push(node.name.clone());
//    if let Some(children) = &node.children {
//        for child in children {
//            extract_names(child, names);
//        }
//    }
//}

fn load_file_modules(path: &Path) -> Result<ModuleFile, Box<dyn std::error::Error>> {
    // Load the modules
    let modules_file =
        File::open(path).unwrap_or_else(|_| panic!("Failed to open module file {:#?}", path));
    let modules_file_data: ModuleFile =
        serde_yaml::from_reader(modules_file).expect("Unable to parse modules");
    Version::parse(&modules_file_data.version).expect("Version is not a valid semver version");

    info!("{} All Modules in the modules file are valid.", OK);
    Ok(modules_file_data)
}

fn load_file_tools(path: &Path) -> Result<ToolFile, Box<dyn std::error::Error>> {
    // Load the tools
    let tools_file = File::open(path).expect("Unable to open file");
    let tools_file_data: ToolFile =
        serde_yaml::from_reader(tools_file).expect("Unable to parse tools");
    Version::parse(&tools_file_data.version).expect("Version is not a valid semver version");

    info!("{} All Tools in the tools file are valid.", OK);
    Ok(tools_file_data)
}

fn load_file_nodes(path: &Path) -> Result<KnownNodesFile, Box<dyn std::error::Error>> {
    // Load the nodes
    let nodes_file = File::open(path).expect("Unable to open file");
    let nodes_file_data: KnownNodesFile =
        serde_yaml::from_reader(nodes_file).expect("Unable to parse nodes");
    Version::parse(&nodes_file_data.version).expect("Version is not a valid semver version");

    debug!("Nodes Library file has parsed correctly");
    Ok(nodes_file_data)
}

fn dependencies_abbr(modules_file_data: &ModuleFile, tools_file_data: &ToolFile) -> Vec<String> {
    // modules and tools are the same, so we need to collect both their abbr in a single vector
    let mut abbrs = Vec::new();
    for (name, _) in modules_file_data.content.iter() {
        abbrs.push(name.clone());
    }
    for (name, _) in tools_file_data.content.iter() {
        abbrs.push(name.clone());
    }
    debug!("Modules and Tools in Library: {:?}", &abbrs);
    abbrs
}

/// Check if the dependencies of a module are in the library
fn validate_nodes_library(
    nodes_file_data: &KnownNodesFile,
    known_dependencies: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    // Throw an error if a node is using a module that is not known
    for (name, node) in nodes_file_data.content.iter() {
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
    info!("{} All Nodes in the nodes file are valid.", OK);
    Ok(())
}

fn name_and_version_from_path(path: &Path) -> (String, Version) {
    //let path = Path::new(path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let file_name_no_ext = file_name.split(".yaml").collect::<Vec<&str>>()[0].to_string();
    let name_version = file_name_no_ext.split('-').collect::<Vec<&str>>();
    if name_version.len() != 2 {
        panic!("Invalid file name: {}", file_name);
    }
    let version = Version::parse(name_version[1]).unwrap();
    let name = name_version[0].to_string();
    debug!("{} file found (version {})", name, version);
    (name, version)
}

fn list_files_in_dir(path: &Path) -> HashMap<String, (PathBuf, Version)> {
    let mut files = HashMap::new();
    for entry in fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();
        if path.is_file() {
            let (name, version) = name_and_version_from_path(&path);
            files.insert(name, (path.to_path_buf(), version));
        }
    }
    files
}

pub fn load_library(library_path: &Path) -> Result<Library, Box<dyn std::error::Error>> {
    let library_list = list_files_in_dir(library_path);
    trace!("{:#?}", library_list);

    // 1. Load the modules file
    let modules = load_file_modules(&library_list["modules"].0).expect("Failed to load modules");
    let tools = load_file_tools(&library_list["tools"].0).expect("Failed to load tools");
    let known_dependencies = dependencies_abbr(&modules, &tools);

    let nodes = load_file_nodes(&library_list["nodes"].0).expect("Failed to load nodes");
    validate_nodes_library(&nodes, &known_dependencies).expect("Failed to validate nodes library");

    let trees = Vec::new();
    let workflows = Vec::new();
    let mut library = Library {
        modules,
        tools,
        nodes,
        trees,
        workflows,
    };
    // 2. Load the btrees
    let mut btree_dir_path = library_path.to_path_buf();
    btree_dir_path.push("trees");
    // 3. Validate all the behavior trees
    // list all .yaml files in the directory
    library.trees = validate_dir_library(
        &btree_dir_path,
        &library,
        load_a_behaviour_tree,
        validate_btree,
    )
    .expect("Failed to validate behavior trees");
    info!("{} All behavior trees in the library are valid.", OK);
    let tree_names = library
        .trees
        .iter()
        .map(|b| &b.tree.name)
        .collect::<Vec<&String>>();
    debug!("List of behavior trees: {:?}", tree_names);
    // 4. Load the worflows
    let mut workflow_dir_path = library_path.to_path_buf();
    workflow_dir_path.push("workflows");
    // 5. Validate all the workflows
    library.workflows = validate_dir_library(
        &workflow_dir_path,
        &library,
        load_a_workflow,
        validate_workflow,
    )
    .expect("Failed to validate workflows");
    info!("{} All workflows in the library are valid.", OK);

    info!("{} Library loading and validation complete.", DONE);
    Ok(library)
}

fn validate_btree(
    tree_file: &BehaviorTreeFile,
    library: &Library,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if btree name is not a known node
    if library.nodes.content.contains_key(&tree_file.tree.name) {
        return Err(format!(
            "Behavior Tree name {} is already used by a known node",
            tree_file.tree.name
        )
        .into());
    }

    // Check if participants are known
    for participant in &tree_file.participants {
        if !library.modules.content.contains_key(participant) {
            return Err(format!("Participant Module {} is not a known node", participant).into());
        }
    }

    validate_node(&tree_file.tree, library).expect("Failed to validate node");

    debug!("Behavior Tree {} is valid.", tree_file.tree.name);

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
        if !library.nodes.content.contains_key(&tree.name) {
            return Err(format!("Node {} is not a known node", tree.name).into());
        }
    }

    Ok(())
}

pub fn home_library_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut home_dir = dirs::home_dir().expect("Unable to find home directory");
    home_dir.push("tcr");
    home_dir.push("library");
    Ok(home_dir)
}

fn validate_workflow(
    workflow_file: &WorkflowFile,
    library: &Library,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if all nodes are valid
    let know_trees = library
        .trees
        .iter()
        .map(|b| &b.tree.name)
        .collect::<Vec<&String>>();
    debug!("List of trees in the Library: {:?}", &know_trees);
    for step in &workflow_file.workflow {
        if !know_trees.contains(&&step.name) {
            return Err(format!("Node {} is not a known node", step.name).into());
        }
    }
    debug!("Workflow {} is valid.", workflow_file.title);
    Ok(())
}

type LoadLibrary<T> = fn(&str) -> Result<T, Box<dyn Error>>;
type ValidateLibrary<T> = fn(&T, &Library) -> Result<(), Box<dyn Error>>;

fn validate_dir_library<T>(
    dir_path: &Path,
    library: &Library,
    loader: LoadLibrary<T>,
    file_validator: ValidateLibrary<T>,
) -> Result<Vec<T>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.extension().unwrap() == "yaml" {
            // Validate the btree
            let file = loader(path.to_str().unwrap()).expect("Failed to deserialize tree");
            file_validator(&file, library).expect("Failed to validate tree");
            files.push(file);
            trace!("{:?} is valid.", &path);
        }
    }
    Ok(files)
}

pub fn get_workflow_by_title<'a>(
    workflow_name: &str,
    library: &'a Library,
) -> Result<&'a WorkflowFile, Box<dyn std::error::Error>> {
    let workflow = library
        .workflows
        .iter()
        .find(|w| w.title == workflow_name)
        .ok_or(format!("Workflow {} not found", workflow_name))?;
    Ok(workflow)
}

pub fn get_tree_by_name<'a>(
    tree_file_name: &str,
    library: &'a Library,
) -> Result<&'a BehaviorTreeFile, Box<dyn std::error::Error>> {
    let tree = library
        .trees
        .iter()
        .find(|t| t.tree.name == tree_file_name)
        .ok_or(format!("Tree {} not found", tree_file_name))?;
    Ok(tree)
}
