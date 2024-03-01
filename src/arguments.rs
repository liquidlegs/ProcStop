mod win_module;
mod config;
use config::*;
use winapi::shared::ntdef::HANDLE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::PROCESS_TERMINATE;

use win_module::*;
use clap::Parser;
use console::style;
const RC_CONFIG_ENV: &str = "procstop_config";
const RC_CONFIG_NAME: &str = "config.json";

#[derive(Debug, Parser, Clone, Default)]
pub struct Arguments {
  /// Debug mode.
  #[clap(short, long, default_value_if("debug", Some("false"), Some("true")), min_values(0))]
  pub debug: bool,

  /// Disables process termination.
  #[clap(short = 'D', long, default_value_if("disable_proc_termination", Some("false"), Some("true")), min_values(0))]
  pub disable_proc_termination: bool,

  /// Enables verbose messages.
  #[clap(short, long, default_value_if("verbose", Some("false"), Some("true")), min_values(0))]
  pub verbose: bool,
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

  pub fn dprint(&self, text: String) -> () {
    if self.debug.clone() == true {
      println!(
        "{} {} {}",
        style("Debug").bright().red(), style("=>").cyan(), style(text).yellow()
      )
    }
  }

  pub fn eprint(text: String) -> () {
    println!("{}: {}", style("Error").bright().red(), style(text).cyan());
  }

  pub fn init(&self) -> RunningConfig {
    let mut content = String::new();                       // Stores the contents of the config file.
    let mut path = String::new();                          // The path to the config.
    let mut config = RunningConfig::new();

    // Attempts to resolve the path to the config from an environment variable.
    if let Some(p) = get_config_path(RC_CONFIG_ENV) {
      path.push_str(p.as_str());
    }

    // If not possible, the current directory is used instead.
    else {
      path.push_str(RC_CONFIG_NAME);
    }

    // Attempts to load the config file from the specified path.
    if let Some(s) = load_config(path.as_str()) {
      content.push_str(s.as_str());
    }

    // Config file is generated if it does not exist.
    else {
      generate_config_file(RC_CONFIG_NAME);
    }

    // IF the config file was generated, a default initalized version of the config structure is returned.
    if content.len() < 1 {
      return config;
    }

    // The config file is parsed and returned as a structure.
    config = parse_config(content);
    config
  }
  
  pub fn run(&self) -> () {
    let config = self.init();                 // Initalizes program from the config file.
    let winapi = WinProcess::new(self.debug);
    let procs = winapi.get_process_list();         // Gets a list of running processes.

    if procs.len() > 0 {

      // Loops through each processes in the blacklist/witelist in the config file.
      for item in config.proccess_list {

        // Loops through each process name returned from get_process_list().
        for i in 0..procs.len() {
          let win = winapi.clone();
  
          // Gets the process name and path.
          let name = win.get_module_name(procs[i] as u32);
          let path = win.get_module_path(procs[i], name.as_str());
  
          if self.debug.clone() == true {
            Self::display_line(
              format!("pid: {} name: {} path: {}", procs[i], name, path).as_str(), i as u32
            );
          }

          // Code block checks if the name of the process matches the name in the config file.
          if name.to_lowercase().as_str().contains(item.as_str()) {
            let mut hproc: HANDLE = std::ptr::null_mut();

            // Gets the handle to the specified process.
            if let Some(h) = WinProcess::get_process_handle(PROCESS_TERMINATE, procs[i] as u32) {
              hproc = h;
            }

            if hproc != std::ptr::null_mut() {
              
              if self.verbose.clone() == true {
                println!(
                  "[{}] Killing process {}",
                  style("+").bright().green(),
                  style(name.clone()).cyan()
                );
              }

              // ONly kills processes if the disable_proc_termination flag is set to false.
              if self.disable_proc_termination.clone() == false {

                // Gets the exit code for the process and kills it if it has suffcient permissions.
                let code = win.get_exit_code(hproc, name.clone().as_str());
                let success = win.kill_process(hproc, code, name.clone().as_str());

                if success == true {
                  println!(
                    "[{}] Successfully killed process {} with pid -> {}",
                    style("+").bright().green(),
                    style(name.clone()).cyan(),
                    style(procs[i]).yellow()
                  );
                }
              }

              // Closes the handle.
              unsafe {
                CloseHandle(hproc)
              };
            }
          }
        }
      }
      
    }
    else {
      println!("{}: unable to retrieve a list of system processes", style("Error").red());
    }
  }
}