use std::io;
use std::mem;
use winapi::shared::minwindef::{DWORD, FALSE};

mod process;
use process::Process;

fn main() {
    let mut success = 0;
    let mut failed = 0;
    enum_proc().unwrap().into_iter().for_each(|pid| match Process::open(pid) {
        Ok(_) => success += 1,
        Err(_) => failed += 1,
    });

    eprintln!("Successfully opened {}/{} processes", success, success + failed);
}

pub fn enum_proc() -> io::Result<Vec<u32>> {
    let mut pids: Vec<u32> = Vec::<DWORD>::with_capacity(1024);
    let mut size: u32 = 0;
    
    // collect all processes from winapi
    if unsafe {
       winapi::um::psapi::EnumProcesses(
pids.as_mut_ptr(),
            //capacity is in bytes so must multiply by the size of DWORD to get proper size
        (pids.capacity() * mem::size_of::<DWORD>()) as u32, 
&mut size,
       )
    } == FALSE 
    {
        return Err(io::Error::last_os_error());
    }

    let count = size as usize / mem::size_of::<DWORD>();
    unsafe {pids.set_len(count);}
    Ok(pids)
}
