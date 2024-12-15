use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Response {
    pub response: Arc<reqwest::Response>,
    pub elapsed: std::time::Duration,
}

#[derive(Debug, Clone)]
pub enum GlobalMessage {
    ResponseReceived(Response),
    ResponseError(Arc<reqwest::Error>),
}
