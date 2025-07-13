use std::io;
use std::mem;
use winapi::shared::minwindef::DWORD;
mod process;
use crate::process::Process;

fn main() {
    print_processes();

    println!("Enter a PID to open:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let pid: u32 = input.trim().parse().expect("Please enter a valid number");

    let process = Process::open(pid).expect("Failed to open process");

    let regions = process.memory_regions().expect("Failed to get memory regions");
    eprintln!("Memory regions found : {}", regions.len());
    if regions.is_empty() {
        eprintln!("No memory regions found.");
    } else {
        for region in &regions {
            eprintln!(
                "Region:
                BaseAddress: {:?}
                AllocationBase: {:?}
                AllocationProtect: {:?}
                RegionSize: {:?}
                State: {:?}
                Protect: {:?}
                Type: {:?}",
                region.BaseAddress,
                region.AllocationBase,
                region.AllocationProtect,
                region.RegionSize,
                region.State,
                region.Protect,
                region.Type,
            );
        }
        eprint!("Memory regions: {:?}", regions.len());
    }
}

pub fn enum_proc() -> io::Result<Vec<u32>> {
    let mut pids: Vec<u32> = Vec::with_capacity(1024);
    let mut size: u32 = 0;

    // collect all processes from winapi
    if unsafe {
        winapi::um::psapi::EnumProcesses(
            pids.as_mut_ptr(),
            // capacity is in bytes so must multiply by the size of DWORD to get proper size
            (pids.capacity() * mem::size_of::<DWORD>()) as u32,
            &mut size,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }

    let count = size as usize / mem::size_of::<DWORD>();
    unsafe {
        pids.set_len(count);
    }
    Ok(pids)
}

pub fn print_processes() {
    enum_proc()
        .unwrap()
        .into_iter()
        .for_each(|pid| match Process::open(pid) {
            Ok(proc) => match proc.name() {
                Ok(name) => println!("{}: {}", pid, name),
                Err(e) => println!("{}: (failed to get name: {})", pid, e),
            },
            _ => {}
        });
}