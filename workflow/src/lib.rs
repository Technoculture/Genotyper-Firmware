pub struct Command {
    pub name: String,
    pub func: dyn FnOnce() -> Result<(), String>,
}

pub struct Workflow {
    pub name: String,
    steps: Vec<Step>,
}

impl Workflow {
    pub fn new(name: String, steps: Vec<Step>) -> Self {
        Self { name, steps }
    }

    pub fn run(&self) {
        for step in &self.steps {
            println!("Running step: {}", step.name);
            println!("Command: {}", step.command);
        }
    }
}

pub struct Step {
    pub name: String,
    command: String,
}

impl Step {
    pub fn new(name: String, command: String) -> Self {
        Self { name, command }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow() {
        let steps = vec![
            Step::new("Step 1".to_string(), "echo 'Hello World'".to_string()),
            Step::new("Step 2".to_string(), "echo 'Hello World'".to_string()),
        ];
        let workflow = Workflow::new("Test Workflow".to_string(), steps);
        let _workflow1 = Workflow {
            name: "Test Workflow".to_string(),
            steps: vec![
                Step::new("Step 1".to_string(), "echo 'Hello World'".to_string()),
                Step::new("Step 2".to_string(), "echo 'Hello World'".to_string()),
            ],
        };
        workflow.run();
    }
}
