use std::boxed::Box;
use std::path::Path;
use console::style;
use std::fs::OpenOptions;
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;

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

/**
 * Function resolves the config path from a custom environment variable.
    Params:
      env_key: &str [The environment variable name]
    Returns Option<String>
 */
pub fn get_config_path(env_key: &str) -> Option<String> {
  let mut path = String::new();

  if let Some(e) = std::env::var_os(env_key) {
    if let Ok(p) = e.into_string() {
      path.push_str(p.as_str());
    }
  }

  else {
    return None;
  }

  Some(path)
}

/**
 * Function loads the config file and returns it as a string.
    Params:
      path: &str [The path to the config file.]
    Returns Option<String>
 */
pub fn load_config(path: &str) -> Option<String> {
  let mut content = String::new();

  match std::fs::read(path) {
    Ok(s) => {
      
      match String::from_utf8(s) {
        Ok(s) => {
          content.push_str(s.as_str());
          
          println!(
            "[{}] Successfully read config file into memory with {} bytes",
            style("+").bright().green(),
            style(content.len()).cyan()
          );
        }

        Err(e) => {
          println!("{}: {}", style("Error").bright().red(), style(format!("{e}")).cyan());
          return None;
        }
      }
    }

    Err(e) => {
      println!("{}: {}", style("Error").bright().red(), style(format!("{e}")).cyan());
      return None;
    }
  }

  Some(content)
}

/**
 * Function writes the content of a string to a file.
    Params:
      content:    String [The bytes to write to a file.]
      file_path:  &str   [The path to the config file.]
    Returns bool
 */
pub fn save_config(content: String, file_path: &str) -> bool {
  let pbuf = Path::new(file_path);
  let mut out = false;
  let mut buffer: Vec<u8> = Default::default();

  for i in content.chars() {
    buffer.push(i as u8);
  }
  
  if pbuf.exists() == true {
    
    match &mut OpenOptions::new().write(true).open(file_path) {
      Ok(s) => {
        if let Ok(size) = s.write(&buffer) {
          println!(
            "[{}] Successfully saved config with {} bytes",
            style("+").bright().green(), style(size).cyan()
          );

          out = true;
        }
      }

      Err(e) => {
        println!("{}: {}", style("Error").bright().red(), style(format!("{e}")).cyan());
      }
    }
  }

  out
}

/**
 * Function creates a new thread and parses the content of the config and returns it as a structure.
    Params:
      content: String [The content to parse.]
    Returns RunningConfig
 */
pub fn parse_config(content: String) -> RunningConfig {
  let mut config = RunningConfig::new();

  // Creates a channel sender and receiver.
  let (tx, rx) = mpsc::channel::<RunningConfig>();
  
  // Creating a new thread here prevents any situations where if there is too much content in the config file
  // that it does not create overflow the main thread, thus causing a stack overflow.
  std::thread::spawn(Box::new(move || {
    
    match serde_json::from_str::<RunningConfig>(&content) {
      Ok(s) => {
        if let Err(e) = tx.send(s) {
          println!(
            "{}: unable to parse config file -  {}",
            style("Error").bright().red(),
            style(format!("{e}")).cyan()
          );
        }
      }
      
      Err(e) => {
        println!("{}: {}", style("Error").bright().red(), style(format!("{e}")).cyan());
      }
    }
  }));

  // After the content is parsed the worker thread is terminated and the main thread receives the structure.
  match rx.recv() {
    Ok(s) => { config = s; }
    Err(e) => {
      println!(
        "{}: unable to received parsed config file over thread - {}",
        style("Error").bright().red(),
        style(e).bright().red()
      );
    }
  }

  config
}

/**
 * Function generates a config file by creating a new structure of the RunningConfig struct.
   Serde_json is then used to convert this into the json format which is then written to the disk as a file.
    Params:
      filename: &str [The name of the config file/]
   Returns nothing
 */
pub fn generate_config_file(filename: &str) -> () {
  let config = RunningConfig::new();
  let mut out = String::new();
  let mut out_content: Vec<u8> = vec![];

  if let Ok(s) = serde_json::to_string_pretty(&config) {
    out.push_str(s.as_str());
  }

  for i in out.chars() {
    out_content.push(i as u8);
  }

  match &mut OpenOptions::new().write(true).create(true).open(filename) {
    Ok(s) => {
      if let Ok(content) = s.write(&out_content) {
        println!(
          "[{}] Successfully wrote config file to disk {} bytes",
          style("+").bright().green(),
          style(content).cyan()
        );
      }
    }

    Err(_) => {}
  }
}