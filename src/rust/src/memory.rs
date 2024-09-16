use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;

use winapi::shared::minwindef::{DWORD, FALSE, LPCVOID, LPVOID};
use winapi::shared::ntdef::NULL;
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32Next, TH32CS_SNAPPROCESS};
use winapi::um::tlhelp32::PROCESSENTRY32;
use winapi::um::winnt::{HANDLE, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

pub struct Memory {
    handle: HANDLE,
}

impl Memory {
    pub fn new(app_name: String) -> Self {
        let mut pid: DWORD = 0;
        let mut entry: PROCESSENTRY32 = unsafe { mem::zeroed() };
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;
        
        // Walk through processes, grab pid
        unsafe {
            let snap_shot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            while Process32Next(snap_shot, &mut entry) != 0 {
                let exe_file = CStr::from_ptr(entry.szExeFile.as_ptr() as *const c_char).to_string_lossy();

                if app_name == exe_file {
                    pid = entry.th32ProcessID;
                }
            }
            CloseHandle(snap_shot);
        }

        if pid == 0 {
            panic!("Process not found");
        }

        let handle = unsafe { OpenProcess(PROCESS_VM_WRITE | PROCESS_VM_READ | PROCESS_VM_OPERATION, FALSE, pid) };

        if handle == NULL {
            panic!("Could not open handle")
        }

        Memory { handle }
    }

    pub fn read_mem<T: Default>(&self, addr: usize) -> T {
        unsafe {
            let mut ret: T = Default::default();
            ReadProcessMemory(self.handle,
                              addr as *mut _,
                              &mut ret as *mut T as LPVOID,
                              std::mem::size_of::<T>(),
                              NULL as *mut usize
            );
            return ret;
        }
    }
    pub fn write_mem<T: Default>(&self, addr: usize, value: &mut T) {
        unsafe {
            WriteProcessMemory(self.handle,
                               addr as *mut _,
                               value as *mut T as LPCVOID,
                               std::mem::size_of::<T>(),
                               NULL as *mut usize
            );
        }
    }
}
impl Drop for Memory {
    fn drop(&mut self) {
        if self.handle != NULL {
            unsafe { CloseHandle(self.handle) };
        }
    }
}