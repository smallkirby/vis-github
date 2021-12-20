use serde::{Serialize, Deserialize};
use crate::context::Context;
use super::client::GithubClient;
use chrono::{DateTime, NaiveDateTime, Local, prelude::*};
use std::process;

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

fn fetch_ratelimit(context: &Context) -> Result<Ratelimit, String> {
  let client = GithubClient::new("rate_limit", &context.apitoken);
  let response = client.get()?;
  let ratelimit: RatelimitWrapper = response.json().unwrap();
  Ok(ratelimit.rate)
}

pub fn show_ratelimit(context: &Context) {
  match fetch_ratelimit(context) {
    Ok(ratelimit) => {
      let user = if context.owner.is_empty() { "(not authorized)" } else { &context.owner };
      println!("User      : {}", user);
      println!("Remaining : {}", ratelimit.remaining);
      println!("Used      : {}", ratelimit.used);
      println!("Reset     : {}", ratelimit.date_reset());
    }
    Err(err) => {
      println!("[ERROR] {}", err);
      process::exit(1);
    },
  }
}
