use super::client::GithubClient;
use crate::context::Context;
use chrono::{prelude::*, DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ratelimit {
  pub limit: u64,
  pub used: u64,
  pub remaining: u64,
  pub reset: i64, // epoch time when ratelimit is cleared
}

impl Ratelimit {
  pub fn date_reset(self) -> DateTime<Local> {
    let naive = NaiveDateTime::from_timestamp(self.reset, 0);
    let local = Local.from_utc_datetime(&naive);
    local
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct RatelimitWrapper {
  rate: Ratelimit,
}

pub fn fetch_ratelimit(context: &Context) -> Result<Ratelimit, String> {
  let client = GithubClient::new("rate_limit", &context.apitoken);
  let response = client.get()?;
  let ratelimit: RatelimitWrapper = response.json().unwrap();
  Ok(ratelimit.rate)
}
