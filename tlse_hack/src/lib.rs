#![cfg(windows)]
#![allow(non_snake_case, unused_variables)]

pub mod loc;

use loc::*;

use std::ptr::null_mut;
use std::io::{Write,BufRead};
use std::convert::TryInto;

// use winapi::ctypes::*;

use winapi::shared::minwindef::*;
// use winapi::shared::windef::*;

use winapi::um::processthreadsapi::*;
use winapi::um::winnt::*;
use winapi::um::consoleapi::*;
use winapi::um::wincon::*;
// use winapi::um::winuser::*;
use winapi::um::memoryapi::*;

#[no_mangle]
unsafe extern "system" fn DllMain(dll_handle: HINSTANCE, fdv_reason: DWORD, lpv_reserved: LPVOID) -> BOOL {
    match fdv_reason {
        DLL_PROCESS_ATTACH => {
            AllocConsole();

            // println!("maybe dev frontend {}", read(G_SHOW_DEV_FRONTEND, 1)[0]);
            // write(G_SHOW_DEV_FRONTEND, &[ 1 ]);
            // println!("maybe dev frontend {}", read(G_SHOW_DEV_FRONTEND, 1)[0]);

            println!("G_FULL_SCREEN {}", read(G_FULL_SCREEN, 1)[0]);
            println!("G_ANTIALIASING_ON {}", read(G_ANTIALIASING_ON, 1)[0]);
            println!("G_ANTIALIASING_X9 {}", read(G_ANTIALIASING_X9, 1)[0]);
            println!("G_KICKOFF_SIZE {}", u32::from_le_bytes(read(G_KICKOFF_SIZE, 4).try_into().unwrap()));
            println!("G_PUSH_BUFFER_SIZE {}",u32::from_le_bytes( read(G_PUSH_BUFFER_SIZE, 4).try_into().unwrap()));
            println!("G_RESOLUTION_REFRESH_RATE {}", u32::from_le_bytes( read(G_RESOLUTION_REFRESH_RATE, 4).try_into().unwrap()));
            println!("G_Z_DEPTH_BUFFER {}", u32::from_le_bytes(read(G_Z_DEPTH_BUFFER, 4).try_into().unwrap()));
            println!("G_RESOLUTION_WIDTH {}", u32::from_le_bytes(read(G_RESOLUTION_WIDTH, 4).try_into().unwrap()));
            println!("G_RESOLUTION_DEPTH {}", u32::from_le_bytes(read(G_RESOLUTION_DEPTH, 4).try_into().unwrap()));
            println!("G_PRESENT_IMMEDIATE {}", read(G_PRESENT_IMMEDIATE, 1)[0]);
            println!("G_RESOLUTION_HEIGHT {}", u32::from_le_bytes(read(G_RESOLUTION_HEIGHT, 4).try_into().unwrap()));

            write(G_FULL_SCREEN, &[ 0 ]);

            println!("flush instruction cache {}", FlushInstructionCache(GetCurrentProcess(), null_mut(), 0));

            CreateThread(null_mut(), 0, Some(init), null_mut(), 0, null_mut());
        },
        DLL_PROCESS_DETACH => {
            FreeConsole();
        },
        _ => {}
    }

    1
}

// unsafe extern "system" fn init(lpThreadParameter: LPVOID) -> DWORD {
//     AllocConsole();

//     // println!("maybe dev frontend {}", read(G_SHOW_DEV_FRONTEND, 1)[0]);
//     // write(G_SHOW_DEV_FRONTEND, &[ 1 ]);
//     // println!("maybe dev frontend {}", read(G_SHOW_DEV_FRONTEND, 1)[0]);

//     println!("fullscreen {}", read(G_FULL_SCREEN, 1)[0]);
//     write(G_FULL_SCREEN, &[ 0 ]);
//     println!("fullscreen {}", read(G_FULL_SCREEN, 1)[0]);

//     println!("width {}", u32::from_le_bytes(read(G_RESOLUTION_WIDTH, 4).try_into().unwrap()));
//     println!("height {}", u32::from_le_bytes(read(G_RESOLUTION_HEIGHT, 4).try_into().unwrap()));
//     println!("refresh rate {}", u32::from_le_bytes(read(G_RESOLUTION_REFRESH_RATE, 4).try_into().unwrap()));

//     println!("flush instruction cache {}", FlushInstructionCache(GetCurrentProcess(), null_mut(), 0));

//     run_prompt();

//     0
// }

unsafe extern "system" fn init(lpThreadParameter: LPVOID) -> DWORD {
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
            "dbg_profile" => {
                if read(G_ALLOW_DEBUG_PROFILE, 1)[0] == 0 {
                    write(G_ALLOW_DEBUG_PROFILE, &[ 1 ]);
                    write(C_USER_PROFILE_MANAGER__IS_DEBUG_PROFILE, &[ 0xb0, 0x01 ]);
                    println!("enabled");
                } else {
                    write(G_ALLOW_DEBUG_PROFILE, &[ 0 ]);
                    write(C_USER_PROFILE_MANAGER__IS_DEBUG_PROFILE, &[ 0x32, 0xc0 ]);
                    println!("disabled");
                }
            },
            _ => println!("Unknown command."),
        }
    }
}

// unsafe fn run_prompt() {
//     let mut stdout = std::io::stdout();

//     let stdin = std::io::stdin();
//     let mut lines = stdin.lock().lines();

//     loop {
//         print!("> ");

//         stdout.flush().unwrap();

//         let line = lines.next().unwrap().unwrap();

//         match line.as_ref() {
//             // ...
//             "" => println!("No command given."),
//             "dbg_profile" => {
//                 if read(G_ALLOW_DEBUG_PROFILE, 1)[0] == 0 {
//                     write(G_ALLOW_DEBUG_PROFILE, &[ 1 ]);
//                     write(C_USER_PROFILE_MANAGER__IS_DEBUG_PROFILE, &[ 0xb0, 0x01 ]);
//                     println!("enabled");
//                 } else {
//                     write(G_ALLOW_DEBUG_PROFILE, &[ 0 ]);
//                     write(C_USER_PROFILE_MANAGER__IS_DEBUG_PROFILE, &[ 0x32, 0xc0 ]);
//                     println!("disabled");
//                 }
//             },
//             _ => println!("Unknown command."),
//         }
//     }
// }

unsafe fn read<'a>(address: usize, length: usize) -> &'a [u8] {
    std::slice::from_raw_parts(address as *mut u8, length)
}

unsafe fn write(address: usize, buffer: &[u8]) {
    let len = buffer.len();

    let mut protect: u32 = 0;
    VirtualProtectEx(GetCurrentProcess(), address as LPVOID, len, PAGE_EXECUTE_READWRITE, &mut protect);

    std::ptr::copy(buffer.as_ptr(), address as *mut u8, len);

    VirtualProtectEx(GetCurrentProcess(), address as LPVOID, len, protect, null_mut());
}

// unsafe fn write_restore(address: usize, buffer: &[u8]) -> impl Fn() {
//     let mut restore: Vec<u8> = Vec::with_capacity(buffer.len());
//     restore.copy_from_slice(buffer);
//     write(address, buffer);
//     move || write(address, &restore)
// }