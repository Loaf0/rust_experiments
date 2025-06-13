use std::io;
use std::ptr::NonNull;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::FALSE;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::handleapi::CloseHandle;

pub struct Process {
    pid: u32,
    handle: NonNull<c_void>,
}

impl Process {
    pub fn open(pid: u32) -> io::Result<Self> {
        // SAFETY: OpenProcess does not have dangerous side-effects.
        let handle = NonNull::new(unsafe { OpenProcess(0, FALSE, pid) });
        handle
            .map(|handle| Self { pid, handle })
            .ok_or_else(io::Error::last_os_error)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        // SAFETY: handle is valid and non-null.
        unsafe { CloseHandle(self.handle.as_ptr()); }
    }
}