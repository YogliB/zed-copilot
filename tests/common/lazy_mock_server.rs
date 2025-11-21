use wiremock::MockServer;

pub struct LazyMockServer {
    server: Option<MockServer>,
}

impl LazyMockServer {
    pub fn new() -> Self {
        LazyMockServer { server: None }
    }

    pub async fn mount_if_needed(&mut self) {
        if self.server.is_none() {
            self.server = Some(MockServer::start().await);
        }
    }

    pub fn server(&mut self) -> &MockServer {
        if self.server.is_none() {
            panic!("MockServer accessed before mount_if_needed() was called");
        }
        self.server.as_ref().unwrap()
    }

    pub fn server_mut(&mut self) -> &mut MockServer {
        if self.server.is_none() {
            panic!("MockServer accessed before mount_if_needed() was called");
        }
        self.server.as_mut().unwrap()
    }

    pub fn uri(&mut self) -> String {
        self.server().uri()
    }
}

impl Default for LazyMockServer {
    fn default() -> Self {
        Self::new()
    }
}
