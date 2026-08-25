use async_trait::async_trait;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::ProgressTx;

#[async_trait]
pub trait AIProvider: Send + Sync {
    fn name(&self) -> &str;

    async fn generate(
        &self,
        prompt: &str,
        progress: Option<&ProgressTx>,
    ) -> Result<String, CliptzyError>;
}
