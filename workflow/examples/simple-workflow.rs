use workflow::Workflow;

fn main() {
    let steps = vec![
        workflow::Step::new("Step 1".to_string(), "echo 'Hello World'".to_string()),
        workflow::Step::new("Step 2".to_string(), "echo 'Hello World'".to_string()),
    ];
    let workflow = Workflow::new("Test Workflow".to_string(), steps);
    workflow.run();
}
