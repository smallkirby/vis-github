#[derive(Debug)]
pub struct Context {
  pub owner: String,
  pub cache_path: String,
  pub force_use_cache: bool,
  pub apitoken: Option<String>,
  pub ignore_fork: bool,
  pub ignore_private: bool,
  pub commit_limit_per_repo: u64,
}