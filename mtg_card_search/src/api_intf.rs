use serde::{Deserialize, Serialize};
use reqwest::Client;
use url::form_urlencoded::{byte_serialize};
use anyhow::Result;
use iced::widget::image::Handle;
use std::sync::Arc;

// Required for Scryfall API
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
  #[serde(default)]
  pub image_uris: ImageUris,
  #[serde(skip)]
  pub image_handle: Option<Handle>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImageUris {
  #[serde(rename = "png")]
  pub png: Option<String>,
}

// Entry point
pub async fn fetch_card (card_name: String) -> Result<Card> {
  let client = build_client()?;

  let url = get_url(&card_name)?;

  // Perform the request
  let response = client
      .get(url)
      .send()
      .await?;

  // Print info
  println!("Status: {}", response.status());
  let json= response.text().await?;
  let mut card: Card = serde_json::from_str(&json)?;
  if let Some(url) = card.image_uris.png.clone() {
    match download_image_handle(client.clone(), url).await {
      Ok(handle) => {
        card.image_handle = Some(handle);
        println!("Downloaded PNG image!");
      }
      Err(err) => {
        eprintln!("Failed to download PNG image: {}", err);
      }
    }
  }
  else {
    println!("Card has no PNG image URL");
  }


  Ok(card)
}

async fn download_image_handle(client: Arc<Client>, url: String) -> Result<Handle> {
  let response = client.get(&url).send().await?;
  let bytes = response.bytes().await?;
  Ok(Handle::from_bytes(bytes))
}

// Build the client
fn build_client() -> Result<Arc<Client>> {
    let client = reqwest::Client::builder()
      .user_agent(APP_USER_AGENT)
      .build()?;
    Ok(Arc::new(client))
}

fn get_url(card_name: &str) -> Result<String> {

  let card_encoded: String = byte_serialize(card_name.trim().as_bytes()).collect();
  Ok(format!("https://api.scryfall.com/cards/named?exact={card_encoded}"))  
}