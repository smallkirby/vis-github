use serde::{Serialize, Deserialize};
use crate::context::Context;
use super::client::GithubClient;
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ratelimit {
  pub limit: u64,
  pub used: u64,
  pub remaining: u64,
  pub reset: i64, // epoch time when ratelimit is cleared
}

impl Ratelimit {
  pub fn date_reset(self) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(self.reset, 0);
    DateTime::from_utc(naive, Utc)
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct RatelimitWrapper {
  rate: Ratelimit,
}

pub fn fetch_ratelimit(_context: &Context) -> Result<Ratelimit, String> {
  let client = GithubClient::new("rate_limit");
  let response = client.get()?;
  let ratelimit: RatelimitWrapper = response.json().unwrap();
  Ok(ratelimit.rate)
}