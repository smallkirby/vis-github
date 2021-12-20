use reqwest::header::{USER_AGENT, AUTHORIZATION};

const BASEURL: &str = "https://api.github.com";

pub struct GithubClient {
  path: String,
  client: reqwest::blocking::RequestBuilder,
}

impl GithubClient {
  pub fn new(path: &str, token: &Option<String>) -> GithubClient {
    let client = reqwest::blocking::Client::new();
    println!("{}", format!("{}/{}", BASEURL, path)); // XXX
    let mut builder = client.get(format!("{}/{}", BASEURL, path))
        .header(USER_AGENT, "vis-github");
    if let Some(token_str) = token {
      builder = builder.header(AUTHORIZATION, format!("token {}", token_str));
    }

    GithubClient {
      path: path.into(),
      client: builder,
    }
  }

  pub fn get(self) -> Result<reqwest::blocking::Response, String> {
    let client = self.client;
    let _result = client.send();
    match _result {
      Ok(result) => {
        if result.status().is_success() {
          Ok(result)
        } else {
          Err(format!("Error code for {} API: {}", self.path, result.status()))
        }
      }
      Err(_err) => {
        Err(format!("Failed to access API for {}", self.path))
      }
    }
  }
}
