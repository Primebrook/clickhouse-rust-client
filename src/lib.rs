mod structured_array;

use reqwest::Error;
use reqwest::blocking::Client as HTTPClient;
use ndarray::Array2;
pub use structured_array::StructuredArray;

pub fn create_client(username: &str, password: &str, host: &str, port: u32) -> Client {
    Client::new(username.to_string(), password.to_string(), host.to_string(), port)
}

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
        // Doesn't care about response metadata
        let url = String::from(format!("http://{}:{}/?query={}", self.host, self.port, query));

        let response: Result<reqwest::blocking::Response, reqwest::Error> = self.http_client.get(&url)
            .header("X-ClickHouse-User", &self.username)
            .header("X-ClickHouse-Key", &self.password)
            .send();

        match response {
            Ok(res) => { res.text() }
            Err(err) => {
                eprintln!("Error sending request");
                Err(err)
            }
        }
    }

    pub fn query_ndarray(&self, _query: &str) -> Result<StructuredArray<f64>, Error> {
        let values = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.];
        let data = Array2::from_shape_vec((3, 3), values).unwrap();
        let column_names = vec!["col1".to_string(), "col2".to_string(), "col3".to_string()];
        Ok(StructuredArray::new(data, column_names))
    }
}