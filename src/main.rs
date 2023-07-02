use clickhouse_connect::create_client;
use reqwest::Error;
// use serde::{Serialize, Deserialize};


fn main() -> Result<(), Error> {

    let client = create_client("default", "", "localhost", 8123);
    let response_body = client.command("SELECT choice FROM base_measures LIMIT 10");

    match response_body {
        Ok(body) => { println!("{}", body); }
        Err(_) => { eprintln!("Error parsing body"); }
    }

    Ok(())
}
