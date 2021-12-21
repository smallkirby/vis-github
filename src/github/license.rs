use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct License {
  pub name: String,
  pub url: Option<String>,
  pub key: Option<String>,
}
