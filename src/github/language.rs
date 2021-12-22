use super::{client::GithubClient, repo::Repository};
use crate::context::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;

static UNITS: [&str; 9] = ["", "K", "M", "G", "T", "P", "E", "Z", "Y"];

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Language {
  pub name: String,
  pub lines: u64,
}

impl Language {
  pub fn from(name: &str, lines: u64) -> Self {
    Language {
      name: name.into(),
      lines,
    }
  }

  pub fn lines_readable(&self) -> String {
    let mut num = self.lines;
    let mut unit = 0usize;
    loop {
      if num < 1000 {
        break;
      }
      num = num / 1000;
      unit += 1;
    }

    format!("{}{} LOC", num, UNITS[unit])
  }
}

pub fn fetch_languages_from_net(
  context: &Context,
  repo_name: &str,
) -> Result<Vec<Language>, String> {
  let client = GithubClient::new(
    &format!("repos/{}/{}/languages", context.owner, repo_name),
    &context.apitoken,
  );
  let response = client.get()?;
  let languages_value: serde_json::Value = response.json().unwrap();
  match languages_value {
    Value::Object(object_map) => {
      let mut success = true;
      let languages = object_map
        .into_iter()
        .map(|(key, value)| match value {
          Value::Number(lines) => Language::from(&key, lines.as_u64().unwrap()),
          _ => {
            success = false;
            Language {
              ..Default::default()
            }
          }
        })
        .collect();
      match success {
        true => Ok(languages),
        false => Err("Fetched data has unexpected structure.".into()),
      }
    }
    _ => Err("Fetched data from API is not object.".into()),
  }
}

impl Repository {
  // fetch languages information for this repo.
  // this method doesn't return errors.
  pub fn fetch_load_languages(&mut self, context: &Context) {
    self.languages = match fetch_languages_from_net(context, &self.name) {
      Ok(languages) => Some(languages),
      Err(_) => None,
    };
  }
}
