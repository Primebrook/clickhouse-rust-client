use reqwest::Error;
use reqwest::blocking::Client as HTTPClient;

pub struct Client {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub http_client: HTTPClient
}

impl Client {
    pub fn new(username: String, password: String, host: String, port: u32) -> Client {
        let http_client = HTTPClient::new();
        Client { username, password, host, port, http_client }
    }

    pub fn command(&self, query: &str) -> Result<String, Error> {

        let url = String::from(format!("http://{}:{}/?query={}", self.host, self.port, query));

        let response: Result<reqwest::blocking::Response, reqwest::Error> = self.http_client.get(&url)
            .header("X-ClickHouse-User", &self.username)
            .header("X-ClickHouse-Key", &self.password)
            .send();

        match response {
            Ok(res) => { res.text() }
            Err(err) => { Err(err) }
        }
    }
}
