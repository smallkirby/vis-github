use crate::github::{commit::*, language::Language, license::*, repo::*};
use chrono::prelude::*;
use std::collections::HashMap;

pub type CommitTimeMap = HashMap<u32, u64>;
pub type LicenseMap = HashMap<License, u64>;

// XXX now it supports only hour granularity.
pub fn devide_by_time(commits: &Vec<Commit>) -> CommitTimeMap {
  let mut map = HashMap::new();
  for commit in commits {
    let date = commit.commit_date();
    let count = map.entry(date.hour()).or_insert(0);
    *count += 1;
  }

  map
}

pub fn devide_by_license(repos: &Vec<Repository>) -> LicenseMap {
  let NOLICENSE: License = License {
    name: "(not licensed)".into(),
    url: None,
    key: None,
  };

  let mut map = HashMap::new();
  for repo in repos {
    if let Some(license) = &repo.license {
      let count = map.entry(license.clone()).or_insert(0);
      *count += 1;
    } else {
      let count = map.entry(NOLICENSE.clone()).or_insert(0);
      *count += 1;
    }
  }

  map
}

pub fn devide_by_language(repos: &Vec<Repository>) -> Vec<Language> {
  let mut map: Vec<Language> = vec![];
  for repo in repos {
    if let Some(languages) = &repo.languages {
      for language in languages {
        let mut existing = false;
        for existing_lang in &mut map {
          if existing_lang.name == language.name {
            existing_lang.lines += language.lines;
            existing = true;
            break;
          }
        }

        if !existing {
          map.push(language.clone());
        }
      }
    }
  }

  map
}
