mod win_module;
use win_module::*;
use clap::Parser;
use console::style;

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
  
  pub fn run(debug: bool) -> () {
    let winapi = WinProcess::new();
    let procs = winapi.get_process_list();
  
    if procs.len() > 0 {
      for i in 0..procs.len() {
        let win = WinProcess {debug};

        let name = win.get_module_name(procs[i] as u32);
        Self::display_line(format!("pid: {} name: {}", procs[i], name).as_str(), i as u32);
      }
    }
    else {
      println!("{}: unable to retrieve a list of system processes", style("Error").red());
    }
  }
}