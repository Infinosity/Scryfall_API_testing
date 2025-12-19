use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::Error as reqwest_Error;
use std::error::Error;
use url::form_urlencoded::{byte_serialize};

// Required for Scryfall API
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Card {
  pub name: String,
  #[serde(rename = "mana_cost")]
  pub mana_cost: String,
  pub cmc: f32,
  #[serde(rename = "type_line")]
  pub type_line: String,
  #[serde(rename = "oracle_text")]
  pub oracle_text: String,
  pub power: Option<String>,
  pub toughness: Option<String>,
}

// Entry point
pub async fn fetch_card (card_name: &str, card_struct: &mut Card) -> Result<(), Box<dyn std::error::Error>> {
    let client = build_client()?;

    let url = get_url(card_name)?;

    // Perform the request
    let response = client
        .get(url)
        .send()
        .await?;

    // Print info
    println!("Status: {}", response.status());
    let json= response.text().await?;
    let fetched_card: Card = serde_json::from_str(&json)?;
    *card_struct = fetched_card;

    Ok(())
}

// Build the client
fn build_client() -> Result<Client, reqwest_Error> {
    let client = reqwest::Client::builder()
      .user_agent(APP_USER_AGENT)
      .build()?;
    Ok(client)
}

fn get_url(card_name: &str) -> Result<String, Box<dyn Error>> {

  let card_encoded: String = byte_serialize(card_name.trim().as_bytes()).collect();
  Ok(format!("https://api.scryfall.com/cards/named?exact={card_encoded}"))  
}