use dirs;

#[derive(Debug, PartialEq)]
pub enum Command {
  RATE,
  FETCH,
  VIS,
  UNKNOWN,
}

#[derive(Debug, PartialEq)]
pub enum VisualizeType {
  TIME,
  LICENSE,
  LANGUAGE,
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
  pub repo_limit_per_user: u64,
  pub command: Command,
  pub vis_type: VisualizeType,
}

impl Default for Context {
  fn default() -> Self {
    let mut cache_path = dirs::home_dir().unwrap();
    cache_path.push(".vis-cache");
    Context {
      owner: "".into(),
      cache_path: cache_path.to_string_lossy().to_string(),
      force_use_cache: false,
      apitoken: None,
      ignore_fork: true,
      ignore_private: true,
      commit_limit_per_repo: 999,
      repo_limit_per_user: 999,
      command: Command::default(),
      vis_type: VisualizeType::UNKNOWN,
    }
  }
}
