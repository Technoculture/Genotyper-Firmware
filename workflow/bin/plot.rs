use clap::{ArgGroup, Args, Parser};
// use std::error::Error;
use log::{debug, info, trace};
use simplelog::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use workflow::conf::{
    get_tree_by_name,
    // get_workflow_by_title,
    load_library,
    root_library_path,
    // BehaviorTreeFile,
    Library,
    // Node,
    // WorkflowFile,
};

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
#[command(name = "plotflow", about = "Plot a workflow or tree", author, version, long_about = None)]
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
    output_path: Option<PathBuf>,
}

pub mod plotflow {
    use log::{debug, info, trace};
    use std::error::Error;
    use std::fs::File;
    use std::io::Write;

    use workflow::{BehaviorTreeFile, Node, Sequence, WorkflowFile};

    pub enum Input<'a> {
        Tree(&'a BehaviorTreeFile),
        Workflow(&'a WorkflowFile),
    }

    pub fn tree_to_dot(tree: &BehaviorTreeFile) -> Result<String, Box<dyn Error>> {
        let mut dot = format!(
            "digraph \"{}\" {{\n\
        rankdir=LR;\n\
        edge [arrowsize=0.7];\n\
        node [colorscheme=paired12];\n\
        edge [colorscheme=paired12];\n\
        ranksep=0.9;\n\
        nodesep=0.3;\n\n",
            tree.title
        );
        traverse_nodes(&tree.tree, &mut dot);
        dot.push_str("}\n");
        Ok(dot)
    }

    fn traverse_nodes(node: &Node, dot: &mut String) {
        let name = &node.name;
        let is_leaf = node.sequence.is_none();
        let (shape, color, fontcolor, label) = if name.starts_with("is_") {
            ("cds", "1", "2", format!("{}?", name))
        } else if name.ends_with("_error") {
            ("record", "7", "10", name.to_string())
        } else if is_leaf {
            ("ellipse", "3", "black", name.to_string())
        } else {
            ("ellipse", "9", "black", name.to_string())
        };

        dot.push_str(&format!(
            "{} [label=\"{}\", shape={}, color={}, fontcolor={}, fontsize=12, style=\"rounded,filled\", fillcolor={}];\n",
            name, label, shape, color, fontcolor, color
        ));

        if let Some(error) = &node.error {
            trace!("{} has error {}", name, error);
            let error_shape = "record";
            let error_color = "7";
            let error_fontcolor = "6";
            let error_label = error.to_string();
            dot.push_str(&format!(
            "{}_{} [label=\"{}\" shape={} color={} fontcolor={} fontsize=12 style=filled fillcolor={}];\n",
                name, error, error_label, error_shape, error_color, error_fontcolor, error_color
            ));
            dot.push_str(&format!(
                "{} -> {}_{} [color={} style=bold];\n",
                name, name, error, error_color
            ));
        }

        let children = match &node.sequence {
            Some(Sequence::Children(nodes)) | Some(Sequence::Fallback(nodes)) => nodes,
            None => return,
        };

        for child in children {
            dot.push_str(&format!(
                "{} -> {} [style=solid, color=10];\n",
                name, child.name
            ));
            traverse_nodes(child, dot);
        }
    }

    pub fn workflow_to_dot(workflow: &WorkflowFile) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }

    pub fn input_to_dot(input: &Input) -> Result<String, Box<dyn Error>> {
        match input {
            Input::Tree(tree) => tree_to_dot(&tree),
            Input::Workflow(workflow) => workflow_to_dot(&workflow),
        }
    }

    //pub fn input_to_png(input: Input) -> Result<Vec<u8>, Box<dyn Error>> {
    //    let dot = input_to_dot(input)?;
    //    let mut cmd = std::process::Command::new("dot");
    //    cmd.arg("-Tpng");
    //    let mut child = cmd.stdin(std::process::Stdio::piped())
    //        .stdout(std::process::Stdio::piped())
    //        .spawn()?;
    //    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    //    stdin.write_all(dot.as_bytes())?;
    //    let output = child.wait_with_output()?;
    //    Ok(output.stdout)
    //}

    pub fn input_to_png(input: &Input) -> Result<Vec<u8>, Box<dyn Error>> {
        let dot = input_to_dot(input)?;
        let mut cmd = std::process::Command::new("dot");
        cmd.arg("-Tpng");
        let mut child = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn dot process");
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin.write_all(dot.as_bytes())?;
        let output = child.wait_with_output()?;
        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(format!(
                "dot process failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into())
        }
    }

    pub fn save_image_to_file(image_data: Vec<u8>, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(file_path)?;
        file.write_all(&image_data)?;
        Ok(())
    }
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
    debug!("{:#?}", &args);

    // Load the library
    let library_path = match args.library_path {
        Some(path) => PathBuf::new().join(path),
        None => root_library_path().expect("Unable to find library path"),
    };
    let library: Library = load_library(&library_path).expect("Failed to load library");

    //let out = get_workflow_by_title(&args.workflow_name, &library).expect("Failed to get workflow");
    //trace!("{:?}", out);

    if let Some(tree_name) = args.input_file.tree_name {
        let out = get_tree_by_name(&tree_name, &library).expect("Failed to get tree");
        trace!("{:#?}", out);

        let dot = plotflow::input_to_dot(&plotflow::Input::Tree(out))?;
        trace!("{}", dot);

        // Save dot in a text file named output.dot
        let mut file = File::create("output.dot")?;
        file.write_all(dot.as_bytes())?;

        let image_data = plotflow::input_to_png(&plotflow::Input::Tree(out))?;
        plotflow::save_image_to_file(image_data, "tree.png")?;
    }

    Ok(())
}
