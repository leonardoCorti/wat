pub struct WwebJsClient;

impl WwebJsClient {
    pub fn new() -> Self {
        WwebJsClient
    }

    pub fn send_message(&self, to: &str, body: &str) {
        println!("Sending message '{}' to {}", body, to);
    }
}
