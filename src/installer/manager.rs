use crate::installer::step::{InstallStep, StepResult, SubLog};

pub struct InstallManager {
    pub steps: Vec<Box<dyn InstallStep + Send + Sync>>,
}

impl InstallManager {
    pub fn new() -> Self {
        Self { steps: vec![] }
    }

    pub fn add_step(&mut self, step: Box<dyn InstallStep>) {
        self.steps.push(step);
    }

    pub async fn run(
        &self,
        sublog_cb: impl Fn(String) + Send + Sync,
        steplog_cb: impl Fn(String) + Send + Sync,
    ) {
        for step in &self.steps {
            steplog_cb(format!("Starting step: {}", step.name()));
            let result = step.run(&|sublog| {
                sublog_cb(format!("[{}] {}", step.name(), sublog.message))
            }).await;

            if result.success {
                steplog_cb(format!("Step finished successfully: {} - {}", step.name(), result.message));
            } else {
                steplog_cb(format!("Step failed: {} - {}", step.name(), result.message));
                break;
            }
        }
    }
}
