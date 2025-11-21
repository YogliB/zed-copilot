use super::lazy_mock_server::LazyMockServer;
use wiremock::MockServer;

pub struct E2ETestContext {
    lazy_mock_server: LazyMockServer,
}

impl E2ETestContext {
    pub async fn new() -> Self {
        Self {
            lazy_mock_server: LazyMockServer::new(),
        }
    }

    pub async fn mock_server_mut(&mut self) -> &mut MockServer {
        self.lazy_mock_server.mount_if_needed().await;
        self.lazy_mock_server.server_mut()
    }

    #[allow(dead_code)]
    pub async fn mock_server(&mut self) -> &MockServer {
        self.lazy_mock_server.mount_if_needed().await;
        self.lazy_mock_server.server()
    }

    #[allow(dead_code)]
    pub async fn openai_base_url(&mut self) -> String {
        self.lazy_mock_server.mount_if_needed().await;
        self.lazy_mock_server.uri()
    }

    #[allow(dead_code)]
    pub async fn anthropic_base_url(&mut self) -> String {
        self.lazy_mock_server.mount_if_needed().await;
        self.lazy_mock_server.uri()
    }
}
