use crate::installer::step::{InstallStep, StepResult, SubLog};

pub struct InstallManager {
    pub steps: Vec<Box<dyn InstallStep + Send + Sync>>,
}

impl InstallManager {
    pub fn new() -> Self {
        Self { steps: vec![] }
    }

    pub fn add_step(&mut self, step: Box<dyn InstallStep + Send + Sync>) {
        self.steps.push(step);
    }

    pub async fn run(
        &self,
        sublog_cb: impl Fn(String) + Send + Sync,
        steplog_cb: impl Fn(String) + Send + Sync,
        on_step_start_cb: impl Fn(String) + Send + Sync,
        on_step_end_cb: impl Fn(bool) + Send + Sync,
    ) {
        for step in &self.steps {
            on_step_start_cb(step.name().to_string());
            steplog_cb(format!("Starting step: {}", step.name()));

            let result = step
                .run(&|sublog: SubLog| {
                    sublog_cb(format!("[{}] {}", step.name(), sublog.message))
                })
                .await;

            let message = if result.success {
                format!(
                    "Step finished successfully: {} - {}",
                    step.name(),
                    result.message
                )
            } else {
                format!("Step failed: {} - {}", step.name(), result.message)
            };

            steplog_cb(message);
            on_step_end_cb(result.success);

            if !result.success {
                break;
            }
        }
    }
}
