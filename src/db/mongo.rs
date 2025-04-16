use mongodb::{Client, error::Result};

pub async fn connect_mongo(url: &str) -> Result<Client> {
    let client = Client::with_uri_str(url).await?;
    println!("Conectado a MongoDB");
    Ok(client)
}