use clap::{ArgGroup, Args, Parser};
// use std::error::Error;
use log::{debug, trace};
use simplelog::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use workflow::conf::{
    get_tree_by_name,
    get_workflow_by_title,
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
    use log::trace;
    use std::error::Error;
    use std::fs::File;
    use std::io::Write;

    use workflow::{get_tree_by_name, BehaviorTreeFile, Library, Node, Sequence, WorkflowFile};

    pub enum Input<'a> {
        Tree(&'a BehaviorTreeFile),
        Workflow(&'a WorkflowFile),
    }

    fn starting_block(title: String) -> String {
        format!(
            "digraph \"{}\" {{\n\
        rankdir=LR;\n\
        edge [arrowsize=0.6];\n\
        node [colorscheme=rdylbu11];\n\
        edge [colorscheme=rdylbu11];\n\
        ranksep=0.6;\n\
        nodesep=0.25;\n\n",
            title
        )
    }

    pub fn tree_to_dot<'a>(
        tree: &BehaviorTreeFile,
        starting_block: &'a mut String,
    ) -> Result<&'a mut String, Box<dyn Error>> {
        traverse_nodes(&tree.tree, starting_block);
        Ok(starting_block)
    }

    fn traverse_nodes(node: &Node, dot: &mut String) {
        let name = &node.name;
        let is_leaf = node.sequence.is_none();
        let (shape, color, fontcolor, label) = if name.starts_with("is_") {
            ("cds", "8", "black", format!("{}?", name))
        } else if is_leaf {
            ("egg", "5", "black", name.to_string())
        } else {
            ("ellipse", "6", "11", name.to_string())
        };

        dot.push_str(&format!(
            "{} [label=\"{}\", shape={} color={} fontcolor={} fontsize=12 style=\"rounded,filled\" fillcolor={}];\n",
            name, label, shape, color, fontcolor, color
        ));

        if let Some(error) = &node.error {
            trace!("{} has error {}", name, error);
            let error_shape = "record";
            let error_color = "1";
            let error_fontcolor = "white";
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
                "{} -> {} [style=solid color=10];\n",
                name, child.name
            ));
            traverse_nodes(child, dot);
        }
    }

    pub fn workflow_to_dot<'a>(
        workflow: &WorkflowFile,
        starting_block: &'a mut String,
        library: &Library,
    ) -> Result<&'a mut String, Box<dyn Error>> {
        let step_names: Vec<String> = workflow
            .workflow
            .iter()
            .map(|step| step.name.clone())
            .collect();

        // draw the workflow
        let workflow_steps_label = step_names
            .iter()
            .enumerate()
            .map(|(index, action)| format!("{} | <{}> {} ", index + 1, index + 1, action))
            .collect::<Vec<String>>()
            .join("| ");

        starting_block.push_str(&format!(
            "workflow [shape=record, label=\"workflow | {{ {} }}\"];\n",
            workflow_steps_label
        ));

        // draw the trees
        for (index, name) in step_names.iter().enumerate() {
            let tree = get_tree_by_name(name, &library)?;

            // let mut tree_dot = starting_block.clone();
            tree_to_dot(&tree, starting_block)?;

            // draw edges between workflow and trees
            starting_block.push_str(&format!(
                "workflow:{} -> {} [style=solid color=10];\n",
                index + 1,
                name
            ));
        }

        Ok(starting_block)
    }

    pub fn input_to_dot(input: &Input, library: &Library) -> Result<String, Box<dyn Error>> {
        match input {
            Input::Tree(tree) => {
                let mut dot = starting_block(tree.title.clone());
                tree_to_dot(&tree, &mut dot)?;
                dot.push_str("}\n");
                Ok(dot)
            }
            Input::Workflow(workflow) => {
                let mut dot = starting_block(workflow.title.clone());
                workflow_to_dot(&workflow, &mut dot, &library)?;
                dot.push_str("}\n");
                Ok(dot)
            }
        }
    }

    pub fn input_to_png(input: &Input, library: &Library) -> Result<Vec<u8>, Box<dyn Error>> {
        let dot = input_to_dot(input, &library)?;
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

    if let Some(tree_name) = args.input_file.tree_name {
        let out = get_tree_by_name(&tree_name, &library).expect("Failed to get tree");
        trace!("{:#?}", out);

        let dot = plotflow::input_to_dot(&plotflow::Input::Tree(out), &library)?;
        trace!("{}", dot);

        // Save dot in a text file named output.dot
        let mut file = File::create("output_tree.dot")?;
        file.write_all(dot.as_bytes())?;

        let image_data = plotflow::input_to_png(&plotflow::Input::Tree(out), &library)?;
        plotflow::save_image_to_file(image_data, "tree.png")?;
    }

    if let Some(worflow_name) = args.input_file.workflow_name {
        let out = get_workflow_by_title(&worflow_name, &library)?;
        trace!("{:?}", out);

        let dot = plotflow::input_to_dot(&plotflow::Input::Workflow(out), &library)?;
        trace!("{}", dot);

        // Save dot in a text file named output.dot
        let mut file = File::create("output_workflow.dot")?;
        file.write_all(dot.as_bytes())?;

        let image_data = plotflow::input_to_png(&plotflow::Input::Workflow(out), &library)?;
        plotflow::save_image_to_file(image_data, "workflow.png")?;
    }

    Ok(())
}
