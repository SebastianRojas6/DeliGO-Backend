pub struct HttpConnection { }

impl HttpConnection {
    pub fn _client(&self) -> reqwest::Client {
        reqwest::Client::new()
    }
}