use reqwest::Error;
use reqwest::blocking::Client as HttpClient;

pub struct Client {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub http_client: HttpClient
}

impl Client {
    pub fn new(username: String, password: String, host: String, port: u32) -> Client {
        let http_client = HttpClient::new();
        Client { username, password, host, port, http_client }
    }

    pub fn command(&self, query: &str) -> Result<String, Error> {

        let url = String::from(format!("http://{}:{}/?query={}", self.host, self.port, query));

        let response: reqwest::blocking::Response = self.http_client.get(&url)
            .header("X-ClickHouse-User", &self.username)
            .header("X-ClickHouse-Key", &self.password)
            .send()
            .unwrap();

        let body: String = response.text().unwrap();

        Ok(body)
    }
}
