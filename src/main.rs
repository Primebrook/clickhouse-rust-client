mod client;
use client::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {

    let client = Client::new("default".to_string(), "".to_string(), "localhost".to_string(), 8123);
    let response_body = client.command("SELECT * FROM base_measures LIMIT 10");

    match response_body {
        Ok(body) => { println!("{}", body); }
        Err(_) => { eprintln!("Error"); }
    }

    Ok(())
}
