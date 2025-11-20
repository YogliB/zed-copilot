use wiremock::MockServer;

pub struct E2ETestContext {
    pub mock_server: MockServer,
}

impl E2ETestContext {
    pub async fn new() -> Self {
        let mock_server = MockServer::start().await;
        Self { mock_server }
    }

    pub fn openai_base_url(&self) -> String {
        self.mock_server.uri()
    }

    pub fn anthropic_base_url(&self) -> String {
        self.mock_server.uri()
    }
}
