use redis::{Client};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut connection = client.get_connection()?;
    let response = redis::cmd("PING").arg("hello").query::<String>(&mut connection)?;
    println!("{}", response);
    Ok(())
}
