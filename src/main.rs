use crossbeam::channel::{Sender, Receiver, unbounded};
use console::style;
use std::{mem, ptr};
use winapi::ctypes::c_void;
use winapi::um::{
  psapi::{EnumProcessModules, GetModuleBaseNameW, EnumProcesses},
  processthreadsapi::{
    OpenProcess, GetExitCodeProcess, TerminateProcess
  },
  handleapi::CloseHandle,
  winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ}
};
use winapi::shared::{
  ntdef::HANDLE,
  minwindef::HMODULE,
  winerror::ERROR_INVALID_HANDLE
};

pub fn display_line(value: &str, index: u32) -> () {
  if index % 2 == 0 {
    println!("{}", style(value).cyan());
  }
  else if index % 2 == 1 {
    println!("{}", style(value).yellow());
  }
}

/**Function returns the name of a module by its pids. */
fn get_module_name(pid: u32) -> String {
  let mut out = String::new();
  let mut buffer: [u16; 260] = [0u16; 260];

  let mut hproc: HANDLE = unsafe {
    OpenProcess(
      PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 
      0 as i32, 
      pid
    )
  };

  if hproc != ptr::null_mut() || hproc != (ERROR_INVALID_HANDLE as isize) as HANDLE {
    let mut hmod: HMODULE = ptr::null_mut();
    let mut bytes: u32 = 0;

    let mut status = unsafe {
      EnumProcessModules(
        hproc,
        &mut hmod as *mut HMODULE,
        mem::size_of_val(&hmod) as u32,
        &mut bytes as *mut u32
      )
    };

    if status > 0 {
      unsafe {
        GetModuleBaseNameW(
          hproc,
          hmod,
          buffer.as_mut_ptr(),
          buffer.len() as u32
        )
      };

      out.push_str(String::from_utf16_lossy(&buffer).as_str().replace("\0", "").as_str());
    }

    unsafe {
      CloseHandle(hproc);
      CloseHandle((&mut hmod as *mut HMODULE) as *mut c_void);
    }
  }

  unsafe {
    CloseHandle(hproc);
  }
  
  out
}

/*Function gets a list of system processes */
fn get_process_list() -> Vec<u32> {
  let mut out: Vec<u32> = Vec::default();
  let mut procs: [u32; 1024] = [0; 1024];
  let mut bytes: u32 = 0;
  let mut status = 0;

  unsafe {
    status = EnumProcesses(
      procs.as_mut_ptr(),
      mem::size_of_val(&procs) as u32,
      &mut bytes as *mut u32
    );  
  }
  
  if status == 1 {
    let mut brk = false;

    for i in procs {
      if i == 0 && brk == true { break; }
      
      out.push(i);

      if brk == false {
        brk = true;
      }
    }
  }

  out
}

fn main() {
  let procs = get_process_list();
  
  if procs.len() > 0 {
    for i in 0..procs.len() {
      let name = get_module_name(procs[i] as u32);
      display_line(format!("pid: {} name: {}", procs[i], name).as_str(), i as u32);
    }
  }
  else {
    println!("{}: unable to retrieve a list of system processes", style("Error").red());
  }
}
