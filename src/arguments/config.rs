use std::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunningConfig {
  pub mode: String,
  pub proccess_list: Vec<String>,
}

impl RunningConfig {
  pub fn new() -> Self {
    RunningConfig {
      mode: String::from("blacklist"),
      proccess_list: vec![],
    }
  }
}

pub fn load_config(path: &str) -> () {
  
}

pub fn save_config(content: String, file_path: &str) -> () {

}

pub fn parse_config(content: String) -> () {

}

pub fn generate_config_file() -> () {
  let config = RunningConfig::new();
  let mut out = String::new();

  if let Ok(s) = serde_json::to_string_pretty(&config) {
    out.push_str(s.as_str());
  }

  println!("{out}");
}