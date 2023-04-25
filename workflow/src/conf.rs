use log::{
    debug, trace, 
    //info
};
use schemars::{schema_for, JsonSchema};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const DONE: &str = "→ ✅";
const OK: &str = "↓ ✔️";

#[derive(Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Endpoint(String);

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub modules: ModuleFile,
    pub tools: ToolFile,
    pub nodes: KnownNodesFile,
    pub trees: Vec<BehaviorTreeFile>,
    pub workflows: Vec<WorkflowFile>,
}

#[derive(Debug, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleFile {
    file_name: Option<String>,
    title: String,
    description: String,
    pub version: String,
    pub endpoint: String,
    pub content: HashMap<String, Module>,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Module {
    pub info: ModuleInfo,
    pub api: API,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub module_type: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct API {
    pub endpoint: String,
    pub variables: Option<HashMap<String, String>>, // TODO: Later <String, data_type>
    pub services: HashMap<String, Service>,         // TODO Later <<path>/, Service>
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Service(HashMap<RequestType, RequestSchema>);

#[derive(Debug, Hash, PartialEq, Eq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RequestType {
    #[serde(rename = "get")]
    GET,
    #[serde(rename = "post")]
    POST,
    #[serde(rename = "put")]
    PUT,
    #[serde(rename = "delete")]
    DELETE,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestSchema {
    pub summary: String,
    pub timeout: String, // TODO: Parse to Duration
    pub parameters: Option<Vec<ValueSchema>>,
    pub response: Option<Vec<ValueSchema>>,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
pub enum Unit {
    #[serde(rename = "C")]
    CELSIUS,
    // Time
    #[serde(rename = "ms")]
    MILISECONDS,
    #[serde(rename = "s")]
    SECONDS,
    #[serde(rename = "m")]
    MINUTES,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String, // TODO: Parse to DataType
    pub description: String,
    pub unit: Option<Unit>,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Value {
    #[serde(flatten)]
    pub schema: ValueSchema,
    pub value: f64,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolFile {
    file_name: Option<String>,
    pub name: String,
    pub version: String,
    pub content: HashMap<String, Tool>,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    pub name: String,
    pub description: String,
    //#[serde(default)]
    pub pick_up: Option<String>,
    //#[serde(default)]
    pub variants: Option<Vec<Variant>>,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub name: String,
    pub description: Option<String>,
    pub abbr: String,
    pub preffered_when: String,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Condition,
    Action,
    Sequence,
    Error,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnownNodesFile {
    file_name: Option<String>,
    pub title: String,
    pub description: String,
    pub version: String,
    pub content: HashMap<String, KnownNode>,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnownNode {
    #[serde(rename = "type")]
    pub node_type: NodeType,
    #[serde(default)]
    pub zenoh: Option<Zenoh>,
    pub description: String,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum ReplyMode {
    Any,
    All,
    One,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Zenoh {
    pub modules: Vec<String>,
    pub min_reply: ReplyMode,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Sequence {
    Children(Vec<Node>), // HAS
    Fallback(Vec<Node>), // HAS
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Node {
    pub name: String,
    pub step_number: u8,
    pub sequence: Option<Sequence>,
    pub error: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BehaviorTreeFile {
    file_name: Option<String>,
    pub title: String,
    pub version: String,
    pub description: String,
    #[serde(rename = "participant_modules")]
    pub participants: Vec<String>,
    pub tree: Node,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowFile {
    file_name: Option<String>,
    pub title: String,
    pub description: String,
    pub version: String,
    pub workflow: Vec<WorkflowStep>,
    pub parameters: Vec<Value>,
    pub process_tldr: String,
}

#[derive(Debug, PartialEq, Serialize, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowStep {
    pub name: String,
    pub why: String,
}

//fn serialize_behaviour_tree(tree: &BehaviorTreeFile, file_name: &str) -> Result<(), Box<dyn Error>> {
//    let yaml_string = serde_yaml::to_string(&tree).expect("Failed to serialize tree");
//    let mut file = File::create(file_name).expect("Failed to create file");
//    file.write_all(yaml_string.as_bytes()).expect("Failed to write to file");
//    Ok(())
//}

fn load_a_behaviour_tree(file_name: &str) -> Result<BehaviorTreeFile, Box<dyn Error>> {
    let file = File::open(file_name).expect("Failed to open file");
    let tree: BehaviorTreeFile = serde_yaml::from_reader(file).expect("Failed to deserialize tree");
    trace!("{:#?}", tree);
    Ok(tree)
}

fn load_a_workflow(file_name: &str) -> Result<WorkflowFile, Box<dyn Error>> {
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

fn load_file_modules(path: &Path) -> Result<ModuleFile, Box<dyn Error>> {
    // Load the modules
    let modules_file =
        File::open(path).unwrap_or_else(|_| panic!("Failed to open module file {:#?}", path));
    let modules_file_data: ModuleFile =
        serde_yaml::from_reader(modules_file).expect("Unable to parse modules");
    Version::parse(&modules_file_data.version).expect("Version is not a valid semver version");

    debug!("{} All Modules in the modules file are valid.", OK);
    Ok(modules_file_data)
}

fn load_file_tools(path: &Path) -> Result<ToolFile, Box<dyn Error>> {
    // Load the tools
    let tools_file = File::open(path).expect("Unable to open file");
    let tools_file_data: ToolFile =
        serde_yaml::from_reader(tools_file).expect("Unable to parse tools");
    Version::parse(&tools_file_data.version).expect("Version is not a valid semver version");

    debug!("{} All Tools in the tools file are valid.", OK);
    Ok(tools_file_data)
}

fn load_file_nodes(path: &Path) -> Result<KnownNodesFile, Box<dyn Error>> {
    // Load the nodes
    let nodes_file = File::open(path).expect("Unable to open file");
    let nodes_file_data: KnownNodesFile =
        serde_yaml::from_reader(nodes_file).expect("Unable to parse nodes");
    Version::parse(&nodes_file_data.version).expect("Version is not a valid semver version");

    trace!("Nodes Library file has parsed correctly");
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
    trace!("Modules and Tools in Library: {:?}", &abbrs);
    abbrs
}

/// Check if the dependencies of a module are in the library
fn validate_nodes_library(
    nodes_file_data: &KnownNodesFile,
    known_dependencies: &[String],
) -> Result<(), Box<dyn Error>> {
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
    debug!("{} All Nodes in the nodes file are valid.", OK);
    Ok(())
}

fn name_from_path(path: &Path) -> String {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    //println!("file_name: {:#?}", file_name.clone().split(".yaml"));
    let file_name_no_ext = file_name.split(".yaml")
        .next()
        .expect(format!("Invalid file name: {}", file_name).as_str());
    trace!("{} file found.", file_name_no_ext);

    let file_name_no_prefix = file_name_no_ext.split(".").next().expect(
        format!("Invalid file name: {}", file_name_no_ext).as_str(),
    );

    trace!("file_name_no_prefix: {:#?}", file_name_no_prefix);

    file_name_no_prefix.to_string()
}

fn list_files_in_dir(path: &Path) -> HashMap<String, PathBuf> {
    let mut files = HashMap::new();
    for entry in fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();
        if path.is_file() {
            let name = name_from_path(&path);
            files.insert(name, path.to_path_buf());
        }
    }
    trace!("{:#?} files found.", files);
    files
}

pub fn load_library(library_path: &Path) -> Result<Library, Box<dyn Error>> {
    let library_list = list_files_in_dir(library_path);
    trace!("{:#?}", library_list);

    // 1. Load the modules file
    let modules = load_file_modules(&library_list["modules"]).expect("Failed to load modules");
    let tools = load_file_tools(&library_list["tools"]).expect("Failed to load tools");
    let known_dependencies = dependencies_abbr(&modules, &tools);

    let nodes = load_file_nodes(&library_list["nodes"]).expect("Failed to load nodes");
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
    debug!("{} All behavior trees in the library are valid.", OK);
    let tree_names = library
        .trees
        .iter()
        .map(|b| &b.tree.name)
        .collect::<Vec<&String>>();
    trace!("List of behavior trees: {:?}", tree_names);
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
    debug!("{} All workflows in the library are valid.", OK);

    debug!("{} Library loading and validation complete.", DONE);
    Ok(library)
}

fn validate_btree(tree_file: &BehaviorTreeFile, library: &Library) -> Result<(), Box<dyn Error>> {
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
    trace!("Behavior Tree {} is valid.", tree_file.tree.name);
    Ok(())
}

fn validate_node(tree: &Node, library: &Library) -> Result<(), Box<dyn Error>> {
    // Check if the tree is valid
    // 1. If the node has children, check if they are valid
    //  a. If a node has no children, check if it is a known node
    //  b. If all children are valid, the node is valid
    // 2. Else If node has fallback, check if all fallback nodes are valid
    // 3. If all nodes are valid, the tree is valid

    match &tree.sequence {
        Some(sequence) => {
            match sequence {
                Sequence::Children(children) | Sequence::Fallback(children) => {
                    for child in children {
                        validate_node(&child, library).expect("Failed to validate node");
                    }
                }
            }
        }
        None => {
            if !library.nodes.content.contains_key(&tree.name) {
                return Err(format!("Node {} is not a known node", tree.name).into());
            }
        }

    }

    Ok(())
}

pub fn root_library_path() -> Result<PathBuf, Box<dyn Error>> {
    let root_dir = if cfg!(windows) {
        Path::new("C:\\")
    } else {
        Path::new("/")
    };

    let mut library_path = PathBuf::from(root_dir);
    library_path.push("tmp");
    library_path.push("tcr");
    library_path.push("genodatalib");
    library_path.push("library");

    Ok(library_path)
}

fn validate_workflow(
    workflow_file: &WorkflowFile,
    library: &Library,
) -> Result<(), Box<dyn Error>> {
    // Check if all nodes are valid
    let know_trees = library
        .trees
        .iter()
        .map(|b| &b.tree.name)
        .collect::<Vec<&String>>();
    trace!("List of trees in the Library: {:?}", &know_trees);
    for step in &workflow_file.workflow {
        if !know_trees.contains(&&step.name) {
            return Err(format!("Node {} is not a known node", step.name).into());
        }
    }
    trace!("Workflow {} is valid.", workflow_file.title);
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
) -> Result<&'a WorkflowFile, Box<dyn Error>> {
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
) -> Result<&'a BehaviorTreeFile, Box<dyn Error>> {
    let tree = library
        .trees
        .iter()
        .find(|t| t.tree.name == tree_file_name)
        .ok_or(format!("Tree {} not found", tree_file_name))?;
    Ok(tree)
}

pub fn file_type_to_schema<T: JsonSchema>(
    name: String,
    schema_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let file_schema = schema_for!(T);
    // Write this file to root_dir/schema/modules_schema.json
    let mut file_schema_path = schema_path.clone();
    file_schema_path.push(format!("{}_schema.json", name));

    let mut file_schema_file = File::create(file_schema_path)?;
    file_schema_file.write_all(
        serde_json::to_string_pretty(&file_schema)
            .unwrap()
            .as_bytes(),
    )?;

    Ok(())
}

pub fn generate_json_schemas() -> Result<(), Box<dyn Error>> {
    let mut schema_path = root_library_path()?;
    schema_path.push("schema");

    // Create the schema json files for each, modules, nodes, trees, worflows, etc.
    file_type_to_schema::<ModuleFile>("modules".to_string(), schema_path.clone())?;
    file_type_to_schema::<KnownNodesFile>("nodes".to_string(), schema_path.clone())?;
    file_type_to_schema::<BehaviorTreeFile>("trees".to_string(), schema_path.clone())?;
    file_type_to_schema::<WorkflowFile>("workflows".to_string(), schema_path.clone())?;
    file_type_to_schema::<ToolFile>("tools".to_string(), schema_path.clone())?;

    Ok(())
}
