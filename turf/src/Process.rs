use std::io;
use std::ptr::NonNull;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{FALSE, HMODULE};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use std::mem::MaybeUninit;


pub struct Process {
    pid: u32,
    handle: NonNull<c_void>,
    name: String,
}

impl Process {
    pub fn open(pid: u32) -> io::Result<Self> {
        let handle = NonNull::new(unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid) });
        let handle = handle.ok_or_else(io::Error::last_os_error)?;
        let process = Process {
            pid,
            handle,
            name: String::new(),
        };
        let name = process.get_process_name().unwrap_or_else(|_| String::from("Unknown"));
        Ok(Process { name, ..process })
    }

    pub fn name(&self) -> Result<&str, std::io::Error> {
        Ok(&self.name)
    }

    pub fn get_process_name(&self) -> io::Result<String> {
        let mut module = MaybeUninit::<HMODULE>::uninit();
        let mut size = 0;

        if unsafe {
            winapi::um::psapi::EnumProcessModules(
                self.handle.as_ptr(),
                module.as_mut_ptr(),
                std::mem::size_of::<HMODULE>() as u32,
                &mut size,
            )
        } == FALSE {
            return Err(io::Error::last_os_error());
        }
        
        let module = unsafe { module.assume_init() };
        
        // use the handle to get the process name

        let mut buffer = Vec::<u8>::with_capacity(64);
        let length = unsafe {
            winapi::um::psapi::GetModuleBaseNameA(
                self.handle.as_ptr(),
                module,
                buffer.as_mut_ptr().cast(),
                buffer.capacity() as u32,
            )
        };
        if length == 0 {
            return Err(io::Error::last_os_error());
        }

        unsafe { buffer.set_len(length as usize) };
        Ok(String::from_utf8(buffer).unwrap())
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        // SAFETY: handle is valid and non-null.
        unsafe { CloseHandle(self.handle.as_ptr()); }
    }
}
