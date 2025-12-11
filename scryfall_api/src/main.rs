use reqwest::Client;
use reqwest::Error as reqwest_Error;
use serde::{Deserialize, Serialize};
use std::error::Error;


static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Serialize, Deserialize)]
struct Card {
  name: String,
  mana_cost: String,
  cmc: f32,
  type_line: String,
  oracle_text: String,
  power: String,
  toughness: String,
}

fn build_client() -> Result<Client, reqwest_Error> {
  // Build the client
    let client = reqwest::Client::builder()
      .user_agent(APP_USER_AGENT)
      .build()?;
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = build_client()?;

    // Perform the request
    let response = client
        .get("https://api.scryfall.com/cards/named?exact=ureni+of+the+unwritten")
        .send()
        .await?;

    // Print info
    println!("Status: {}", response.status());
    let json= response.text().await?;
    let c: Card = serde_json::from_str(&json)?;
    println!("Name: {}", c.name);

    Ok(())
    
}
