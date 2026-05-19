
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{ GetCurrentProcessId, OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ };

#[cfg(target_os = "windows")]
use windows::Win32::System::ProcessStatus::{ GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS };


pub struct System { }


impl System {

    /// Return the number of bytes used by the process
    /// 
    pub fn get_max_memory() -> u32 {

        #[cfg(target_os = "windows")]
        return Self::get_max_memory_win32();
        
    }

    #[cfg(target_os = "windows")]
    fn get_max_memory_win32() -> u32 {

        unsafe {

            let pid: u32 = GetCurrentProcessId();
    
            let hprocess = match OpenProcess( PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid ) {
                Ok(h) => h,
                Err(_) => return 0,
            };
    
            let mut pmc: PROCESS_MEMORY_COUNTERS = PROCESS_MEMORY_COUNTERS::default();
    
            let result = GetProcessMemoryInfo(hprocess, &mut pmc, size_of::<PROCESS_MEMORY_COUNTERS>() as u32);
    
            if result.is_ok() {
                return pmc.WorkingSetSize as u32;
            }
    
            return 0;

        }

    }

    

}