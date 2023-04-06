#![allow(dead_code)]
use std::thread;

/// Proto
/// Objective:
/// - have a workflow list of tasks
/// - each task is a future
/// - each task contains a behavior tree of tasks
/// - each task in the behavior tree is a future
/// - loop through the workflow list of tasks
/// - when the workflow is complete, abort with a message
/// - if a task or a behavior tree task fails, abort with the error message

#[derive(Debug)]
struct Workflow {
    tasks: Vec<Task>,
}

#[derive(Debug)]
enum ExecutionError {
    ActionFailed(String),
    Cancelled(String),
    WorkflowFailed(String),
}

#[derive(Debug)]
struct Task {
    name: String,
    behavior_tree: BehaviorTreeNode,
}

#[derive(Debug)]
struct Sequence {
    name: String,
    children: Vec<BehaviorTreeNode>,
}

#[derive(Debug)]
enum BehaviorTreeNode {
    Condition(Condition),
    Action(Action),
    Sequence(Sequence),
    Error(ExecutionError),
}

#[derive(Debug)]
struct Action {
    name: String,
    func: fn() -> Result<(), ExecutionError>,
}

#[derive(Debug)]
struct Condition {
    name: String,
    func: fn() -> Result<bool, ExecutionError>,
    fail: Box<BehaviorTreeNode>,
}

fn main() {
    let is_tip_available = Condition {
        name: "is_tip_available".to_string(),
        func: || {
            println!("is_tip_available");
            Ok(true)
        },
        fail: Box::new(BehaviorTreeNode::Error(ExecutionError::ActionFailed("is_tip_available".to_string()))),
    };

    let is_tip_available_node = BehaviorTreeNode::Condition(is_tip_available);

    let get_tip = Action {
        name: "get_tip".to_string(),
        func: || {
            println!("get_tip");
            Ok(())
        },
    };

    let get_tip_node = BehaviorTreeNode::Action(get_tip);

    let sequence = Sequence {
        name: "sequence".to_string(),
        children: vec![is_tip_available_node, get_tip_node],
    };

    let sequence_node = BehaviorTreeNode::Sequence(sequence);

    let task = Task {
        name: "task".to_string(),
        behavior_tree: sequence_node,
    };

    let workflow = Workflow {
        tasks: vec![task],
    };

    let mut workflow = workflow;

    let mut task = workflow.tasks.pop().unwrap();

    let mut behavior_tree = task.behavior_tree;

    let mut sequence = match behavior_tree {
        BehaviorTreeNode::Sequence(sequence) => sequence,
        _ => panic!("Expected BehaviorTreeNode::Sequence"),
    };

    let mut children = sequence.children;

    let mut condition = match children.pop().unwrap() {
        BehaviorTreeNode::Condition(condition) => condition,
        _ => panic!("Expected BehaviorTreeNode::Condition"),
    };

    let result = (condition.func)();

    let mut behavior_tree = match result {
        Ok(true) => {
            let action = match children.pop().unwrap() {
                BehaviorTreeNode::Action(action) => action,
                _ => panic!("Expected BehaviorTreeNode::Action"),
            };

            let result = (action.func)();

            match result {
                Ok(_) => BehaviorTreeNode::Error(ExecutionError::Cancelled("Workflow complete".to_string())),
                Err(error) => BehaviorTreeNode::Error(error),
            }
        },
        Ok(false) => *condition.fail,
        Err(error) => BehaviorTreeNode::Error(error),
    };

    println!("{:#?}", behavior_tree);

    thread::park();
}
