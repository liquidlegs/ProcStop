use console::style;
use std::{mem, ptr};
use winapi::ctypes::c_void;
use winapi::um::{
  psapi::{EnumProcessModules, GetModuleBaseNameW, EnumProcesses, GetModuleFileNameExW},
  processthreadsapi::{
    OpenProcess, GetExitCodeProcess, TerminateProcess
  },
  handleapi::CloseHandle,
  winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
};
use winapi::shared::{
  ntdef::HANDLE,
  minwindef::HMODULE,
  winerror::ERROR_INVALID_HANDLE
};

#[derive(Debug, Clone, Copy)]
pub struct WinProcess {
  pub debug: bool,
}

impl WinProcess {
  pub fn new(debug: bool) -> Self {
    WinProcess { debug: debug }
  }

  pub fn dprint(text: &str, debug: bool) -> () {
    if debug == true {
      println!(
        "{} {} {}",
        style("Debug").bright().red(), style("=>").cyan(), style(text).yellow()
      )
    }
  }

  pub fn get_exit_code(self, hproc: HANDLE, proc_name: &str) -> u32 {
    let dbg = self.debug.clone();

    let mut code: u32 = 0;
    let status = unsafe {
      GetExitCodeProcess(
        hproc,
        &mut code as *mut u32
      )
    };

    if status == 1 {
      Self::dprint(format!("Successfully grabbed exit code for process {}", proc_name).as_str(), dbg.clone());
    }

    else {
      Self::dprint(format!("Failed to grab exit code for process {}", proc_name).as_str(), dbg.clone());
    }
    
    code
  }

  pub fn kill_process(self, hproc: HANDLE, code: u32, proc_name: &str) -> bool {
    let dbg = self.debug.clone();
    let status = unsafe {
      TerminateProcess(hproc, code)
    };

    if status == 1 {
      Self::dprint(format!("Sucessfully killed process {}", proc_name).as_str(), dbg.clone());
      true
    }

    else {
      Self::dprint(format!("Failed to kill process {}", proc_name).as_str(), dbg.clone());
      false
    }
  }

  pub fn get_process_handle(access: u32, pid: u32) -> Option<HANDLE> {
    let mut hproc: HANDLE = ptr::null_mut();
    
    hproc = unsafe {
      OpenProcess(
        access, 
        0 as i32, 
        pid
      )
    };

    if hproc != ptr::null_mut() {
      Some(hproc)
    }

    else {
      None
    }
  }

  pub fn get_module_path(self, pid: u32, proc_name: &str) -> String {
    let mut out = String::new();
    let mut buffer: [u16; 260] = [0u16; 260];
    let dbg = self.debug.clone();

    Self::dprint(
      format!("Opening process pid -> [{}] with read permisions", pid).as_str(), dbg.clone()
    );
    
    let mut hproc: HANDLE = unsafe {
      OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 
        0 as i32, 
        pid
      )
    };

    Self::dprint(
      format!("Attempting to retrieve path for process -> {}", proc_name).as_str(), dbg.clone()
    );

    let length = unsafe {
      GetModuleFileNameExW(
        hproc,
        ptr::null_mut(),
        &mut buffer as *mut u16,
        mem::size_of_val(&buffer) as u32
      )
    };

    if length > 0 {
      Self::dprint(
        format!("Successfully retrieved path for process -> {}", proc_name).as_str(), dbg.clone()
      );

      out.push_str(String::from_utf16_lossy(&buffer).as_str());
    }

    else {
      Self::dprint(
        format!("Failed to retrieve path for process -> {}", proc_name).as_str(), dbg.clone()
      );

      out.push_str("none");
    }

    // Check if the handle is not NULL and give resources back to the system.
    if hproc != ptr::null_mut() {
      unsafe {
        CloseHandle(hproc)
      };
    }
    
    out
  }

  pub fn get_module_name(self, pid: u32) -> String {
    let mut out = String::new();
    let mut buffer: [u16; 260] = [0u16; 260];
    let dbg = self.debug.clone();
  
    Self::dprint(format!("Opening process pid -> [{}] with read permisions", pid).as_str(), dbg.clone());
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
  
      Self::dprint(format!("Enumerating modules in pid -> [{}]", pid).as_str(), dbg.clone());
      let mut status = unsafe {
        EnumProcessModules(
          hproc,
          &mut hmod as *mut HMODULE,
          mem::size_of_val(&hmod) as u32,
          &mut bytes as *mut u32
        )
      };
  
      if status > 0 {
        Self::dprint(format!("Getting module base name for pid -> {}", pid).as_str(), dbg.clone());

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

      else {
        Self::dprint(format!("Failed to get module base name for pid -> [{}]", pid).as_str(), dbg.clone());
        out.push_str("none");
      }
  
      unsafe {
        CloseHandle(hproc);
        CloseHandle((&mut hmod as *mut HMODULE) as *mut c_void);
      }
    }

    else {
      Self::dprint(format!("Process handle for pid -> [{}] is NULL", pid).as_str(), dbg.clone());
    }
  
    unsafe {
      CloseHandle(hproc);
    }
    
    out
  }
  
  /*Function gets a list of system processes */
  pub fn get_process_list(self) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::default();
    let mut procs: [u32; 1024] = [0; 1024];
    let mut bytes: u32 = 0;
    let mut status = 0;
    let dbg = self.debug.clone();
  
    Self::dprint("Enumerating system processes", dbg.clone());
    unsafe {
      status = EnumProcesses(
        procs.as_mut_ptr(),
        mem::size_of_val(&procs) as u32,
        &mut bytes as *mut u32
      );  
    }
    
    if status == 1 {
      Self::dprint("Removing empty processes", dbg.clone());
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
}