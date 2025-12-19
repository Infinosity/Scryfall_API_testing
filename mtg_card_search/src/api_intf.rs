use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::Error as reqwest_Error;
use std::error::Error;
use url::form_urlencoded::{byte_serialize};

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