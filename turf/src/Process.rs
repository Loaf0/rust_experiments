#![allow(unused)]

use std::io;
use std::ptr::NonNull;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{FALSE, HMODULE};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::um::winnt::PROCESS_VM_WRITE;
use std::mem::MaybeUninit;
use winapi::um::winnt::PROCESS_VM_OPERATION;
use crate::mem;
use winapi::um::winnt::{MEM_COMMIT, PAGE_GUARD, PAGE_NOACCESS, MEMORY_BASIC_INFORMATION};


pub struct Process {
    pid: u32,
    handle: NonNull<c_void>,
}

impl Process {
    pub fn open(pid: u32) -> io::Result<Self> {
        NonNull::new(unsafe {
            OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION,
                FALSE,
                pid,
            )
        })
        .map(|handle| Self { pid, handle })
        .ok_or_else(io::Error::last_os_error)
    }

    pub fn name(&self) -> io::Result<String> {
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

    pub fn memory_regions(&self) -> io::Result<Vec<MEMORY_BASIC_INFORMATION>> {
        let mut regions = Vec::new();
        let mut address = 0usize;
        let mut info = std::mem::MaybeUninit::<MEMORY_BASIC_INFORMATION>::uninit();

        eprintln!("Starting memory scan...");

        // Debug: Show process handle
        eprintln!("Process handle: {:?}", self.handle);

        while address < isize::MAX as usize {
            eprintln!("Querying address: 0x{:X}", address);

            let written = unsafe {
                winapi::um::memoryapi::VirtualQueryEx(
                    self.handle.as_ptr(),
                    address as *const _,
                    info.as_mut_ptr(),
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                )
            };

            if written == 0 {
                let err = io::Error::last_os_error();
                eprintln!("VirtualQueryEx failed at 0x{:X}: {}", address, err);
                break;
            }

            let region = unsafe { info.assume_init() };

            eprintln!(
                "Found region: Base=0x{:X}, Size={}, State=0x{:X}, Protect=0x{:X}, Type=0x{:X}",
                region.BaseAddress as usize,
                region.RegionSize,
                region.State,
                region.Protect,
                region.Type,
            );

            if region.State == MEM_COMMIT &&
                (region.Protect & PAGE_GUARD == 0) &&
                (region.Protect & PAGE_NOACCESS == 0) {
                eprintln!("-> Region accepted and added.");
                regions.push(region);
            } else {
                eprintln!("-> Region skipped due to protection or state.");
            }

            // Prevent infinite loops on bad memory region reporting
            if region.RegionSize == 0 {
                eprintln!("-> Region size is 0; breaking to avoid infinite loop.");
                break;
            }

            address = region.BaseAddress as usize + region.RegionSize;
        }

        eprintln!("Completed memory scan. Total regions collected: {}", regions.len());

        Ok(regions)
    }


}

impl Drop for Process {
    fn drop(&mut self) {
        // SAFETY: handle is valid and non-null.
        unsafe { CloseHandle(self.handle.as_ptr()); }
    }
}


