use crate::providers::error::ProviderResult;
use futures::Stream;
use std::pin::Pin;

#[async_trait::async_trait(?Send)]
pub trait AiProvider {
    async fn complete(&self, prompt: &str) -> ProviderResult<String>;

    async fn is_available(&self) -> bool;

    fn name(&self) -> &str;

    fn model(&self) -> &str;

    async fn complete_stream(
        &self,
        prompt: &str,
    ) -> ProviderResult<Pin<Box<dyn Stream<Item = ProviderResult<String>> + Send>>>;
}
