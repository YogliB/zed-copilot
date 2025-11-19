use crate::providers::error::ProviderResult;

#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(&self, prompt: &str) -> ProviderResult<String>;

    async fn is_available(&self) -> bool;

    fn name(&self) -> &str;

    fn model(&self) -> &str;
}
