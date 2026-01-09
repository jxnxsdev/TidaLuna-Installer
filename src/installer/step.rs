use async_trait::async_trait;

pub struct SubLog {
    pub message: String,
}

pub struct StepResult {
    pub success: bool,
    pub message: String,
}

#[async_trait::async_trait]
pub trait InstallStep: Send + Sync {
    fn name(&self) -> &str;

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult;
}