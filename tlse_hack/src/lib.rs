#![cfg(windows)]
#![allow(non_snake_case, unused_variables)]

use std::ptr::null_mut;
use std::io::{Write,BufRead};

use winapi::ctypes::*;

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;

use winapi::um::processthreadsapi::*;
use winapi::um::winnt::*;
use winapi::um::consoleapi::*;
use winapi::um::wincon::*;
use winapi::um::winuser::*;
use winapi::um::memoryapi::*;

#[no_mangle]
unsafe extern "system" fn DllMain(dll_handle: HINSTANCE, fdv_reason: DWORD, lpv_reserved: LPVOID) -> BOOL {
    match fdv_reason {
        DLL_PROCESS_ATTACH => {
            CreateThread(null_mut(), 0, Some(init), null_mut(), 0, null_mut());
        },
        DLL_PROCESS_DETACH => {
            FreeConsole();
        },
        _ => {}
    }

    1
}

unsafe extern "system" fn init(lpThreadParameter: LPVOID) -> DWORD {
    AllocConsole();

    let instr_ptr = 0x00a60093 as *mut [u8; 5];
    let instr_len = (*instr_ptr).len();
    let mut old_protection: u32 = 0;

    println!("instr_ptr {:x?}", instr_ptr);
    println!("instr_ptr len {:?}", instr_len);

    VirtualProtectEx(GetCurrentProcess(), instr_ptr as LPVOID, instr_len, PAGE_EXECUTE_READWRITE, &mut old_protection);
    println!("instr {:x?}", *instr_ptr);

    std::ptr::write_bytes(instr_ptr, 0, instr_len);

    // let style_bytes = WS_BORDER.to_le_bytes();
    // println!("style_bytes {:x?}", style_bytes);
    // (*instr_ptr)[1] = style_bytes[0];
    // (*instr_ptr)[2] = style_bytes[1];
    // (*instr_ptr)[3] = style_bytes[2];
    // (*instr_ptr)[4] = style_bytes[3];

    VirtualProtectEx(GetCurrentProcess(), instr_ptr as LPVOID, instr_len, old_protection, null_mut());

    println!("modified instr {:x?}", *instr_ptr);

    println!("flush instruction cache {}", FlushInstructionCache(GetCurrentProcess(), null_mut(), 0));

    run_prompt()
}

fn run_prompt() {
    let mut stdout = std::io::stdout();

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        print!("> ");

        stdout.flush().unwrap();

        let line = lines.next().unwrap().unwrap();

        match line.as_ref() {
            // ...
            "" => println!("No command given."),
            "toggle_debug_profile" => {
                let debug_profile_ptr = (0x00400000 + 0xF75741) as *mut u8;
                let mut old_protection: u32 = 0;

                VirtualProtectEx(GetCurrentProcess(), debug_profile_ptr as LPVOID, 3, PAGE_EXECUTE_READWRITE, &mut old_protection);

                println!("debug_profile data {}", *debug_profile_ptr);

                if (*debug_profile_ptr) == 0 {
                    (*debug_profile_ptr) = 1;
                } else {
                    (*debug_profile_ptr) = 0;
                }

                println!("debug_profile data {}", *debug_profile_ptr);

                VirtualProtectEx(GetCurrentProcess(), debug_profile_ptr as LPVOID, 3, old_protection, null_mut());

                let debug_profile_ptr = 0x407030 as *mut [u8; 3];
                let mut old_protection: u32 = 0;

                VirtualProtectEx(GetCurrentProcess(), debug_profile_ptr as LPVOID, 3, PAGE_EXECUTE_READWRITE, &mut old_protection);

                // println!("old_protection {:x?}", old_protection);

                println!("debug_profile fn {:x?}", *debug_profile_ptr);

                if (*debug_profile_ptr)[0] == 0x32 {
                    (*debug_profile_ptr)[0] = 0xb0;
                    (*debug_profile_ptr)[1] = 0x01;
                } else {
                    (*debug_profile_ptr)[0] = 0x32;
                    (*debug_profile_ptr)[1] = 0xc0;
                }

                VirtualProtectEx(GetCurrentProcess(), debug_profile_ptr as LPVOID, 3, old_protection, null_mut());

                println!("debug_profile fn {:x?}", *debug_profile_ptr);
            }
            _ => println!("Unknown command."),
        }
    }
}