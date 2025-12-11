use reqwest::Client;
use reqwest::Error as reqwest_Error;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::form_urlencoded::{byte_serialize};
use std::io;


static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Serialize, Deserialize)]
struct Card {
  name: String,
  mana_cost: String,
  cmc: f32,
  type_line: String,
  oracle_text: String,
  power: Option<String>,
  toughness: Option<String>,
}

fn build_client() -> Result<Client, reqwest_Error> {
  // Build the client
    let client = reqwest::Client::builder()
      .user_agent(APP_USER_AGENT)
      .build()?;
    Ok(client)
}

fn get_url() -> Result<String, Box<dyn Error>> {  
  println!("Enter a card name: ");
  let mut card_name = String::new();

  io::stdin().read_line(&mut card_name)?;

  let card_encoded: String = byte_serialize(card_name.trim().as_bytes()).collect();
  let url: String = format!("https://api.scryfall.com/cards/named?exact={}", card_encoded);

  Ok(url)
  
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = build_client()?;

    let url = get_url()?;

    // Perform the request
    let response = client
        .get(url)
        .send()
        .await?;

    // Print info
    println!("Status: {}", response.status());
    let json= response.text().await?;
    let c: Card = serde_json::from_str(&json)?;
    println!("Name: {}\nMana Cost: {}\nText: {}", c.name, c.mana_cost, c.oracle_text);
    //not sure how to do this yet
    // if (c.power) {
    //   print!("Power/Toughness: {}/{}", c.power, c.toughness);
    // }

    Ok(())
    
}
