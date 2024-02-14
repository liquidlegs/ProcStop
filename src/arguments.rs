mod win_module;
mod config;
use config::*;
use std::env;

use win_module::*;
use clap::Parser;
use console::style;
const RC_CONFIG_ENV: &str = "procstop_config";

#[derive(Debug, Parser, Clone, Default)]
pub struct Arguments {
  /// Debug mode
  #[clap(short, long, default_value_if("debug", Some("false"), Some("true")), min_values(0))]
  pub debug: bool
}

impl Arguments {
  fn display_line(value: &str, index: u32) -> () {
    if index % 2 == 0 {
      println!("{}", style(value).bright().cyan());
    }
    else if index % 2 == 1 {
      println!("{}", style(value).bright().yellow());
    }
  }

  pub fn test() -> () {
    if let Some(e) = env::var_os("path") {
      println!("{}", e.into_string().unwrap().as_str());
    }

    generate_config_file()
  }
  
  pub fn run(debug: bool) -> () {
    let winapi = WinProcess::new();
    let procs = winapi.get_process_list();
  
    if procs.len() > 0 {
      for i in 0..procs.len() {
        let win = WinProcess {debug};
        let win_path = WinProcess {debug};

        let name = win.get_module_name(procs[i] as u32);
        let path = win_path.get_module_path(procs[i], name.as_str());
        Self::display_line(
          format!("pid: {} name: {} path: {}", procs[i], name, path).as_str(), i as u32
        );
      }
    }
    else {
      println!("{}: unable to retrieve a list of system processes", style("Error").red());
    }
  }
}