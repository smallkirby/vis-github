use super::separator::*;
use crate::context::*;
use crate::github::{commit::*, repo::*};

use std::collections::HashMap;

pub fn analyze_by_time(context: &Context) -> Result<CommitTimeMap, String> {
  let mut time_map: CommitTimeMap = HashMap::new();

  if context.owner.is_empty() {
    return Err("[ERROR] username not specified.".into());
  }

  let repos = fetch_repositories_from_file(&context.owner, &context.cache_path)?;
  for repo in repos {
    if !repo.is_target(&context) {
      continue;
    }
    let commits = match fetch_commits_from_file(&context.cache_path, &context.owner, &repo.name) {
      Ok(commits) => commits,
      Err(_err) => {
        println!(
          "(skipping repository {} due to error while reading cache)",
          repo.name
        );
        continue;
      }
    };
    let tmp_map = devide_by_time(&commits);
    for (time, count) in tmp_map {
      let counter = time_map.entry(time).or_insert(0);
      *counter += count;
    }
  }

  Ok(time_map)
}

pub fn analyze_by_license(context: &Context) -> Result<LicenseMap, String> {
  if context.owner.is_empty() {
    return Err("[ERROR] username not specified.".into());
  }

  let repos = fetch_repositories_from_file(&context.owner, &context.cache_path)?;
  let target_repos: Vec<Repository> = repos
    .into_iter()
    .filter(|repo| repo.is_target(context))
    .collect();
  Ok(devide_by_license(&target_repos))
}
