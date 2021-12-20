use crate::github::{repo::*, commit::*};
use chrono::prelude::*;
use std::collections::HashMap;

pub type CommitTimeMap = HashMap<u32, u64>;

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
