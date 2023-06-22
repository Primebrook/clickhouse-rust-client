use reqwest::Error;

fn create() {
    println!("Creating test database...");
    println!("Creating respondent table...")
}

fn seed() {
    println!("Seeding respondent table...");
}

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    create();
    seed();
    let query: String = String::from("SELECT divide(1, 2) AS division_col FORMAT Pretty").replace(" ", "%20");
    let response: reqwest::blocking::Response = reqwest::blocking::get(format!("http://localhost:8123/?query={}", query))?;

    println!("Status: {}", response.status());
    println!("Body:\n\n {}", response.text()?);

    Ok(())
}
