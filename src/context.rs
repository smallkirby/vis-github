#[derive(Debug, PartialEq)]
pub enum Command {
  RATE,
  FETCH,
  UNKNOWN,
}

impl Default for Command {
  fn default() -> Self {
    Self::UNKNOWN
  }
}

#[derive(Debug)]
pub struct Context {
  pub owner: String,
  pub cache_path: String,
  pub force_use_cache: bool,
  pub apitoken: Option<String>,
  pub ignore_fork: bool,
  pub ignore_private: bool,
  pub commit_limit_per_repo: u64,
  pub command: Command,
}

impl Default for Context {
  fn default() -> Self {
    Context {
      owner: "".into(),
      cache_path: "~/vis-cache".into(),
      force_use_cache: false,
      apitoken: None,
      ignore_fork: true,
      ignore_private: true,
      commit_limit_per_repo: 999,
      command: Command::default(),
    }
  }
}

