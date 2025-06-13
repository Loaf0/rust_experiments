use std::ptr::NonNull;
use winapi::ctypes::c_void;

pub struct Process {
    pub pid: u32,
    pub handle: NonNull<c_void>,
}

impl Process {
    pub fn open(pid: u32) -> io::Result<Self> {
        todo!("Implement process opening logic");

        NonNull::new(unsafe {winapi::um::processthreadsapi::OpenProcess(0, FALSE, pid)})
            .map(|handle| Self {pid, handle})
            .ok_or_else(||io:Error::last_os_error())
    }
}

impl Drop for Process {
    fn drop(&mut self){
        todo!("Implement process cleanup logic");
    }
}

