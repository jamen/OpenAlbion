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

use tlse_sys::CMainGameComponent;

#[no_mangle]
unsafe extern "system" fn DllMain(dll_handle: HINSTANCE, fdv_reason: DWORD, lpv_reserved: LPVOID) -> BOOL {
    match fdv_reason {
        DLL_PROCESS_ATTACH => {
            AllocConsole();

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
            "test_evts" => {
                let evts_b = *(P_MAIN_GAME_COMPONENT as *mut usize) + 164;
                let evts_e = *(P_MAIN_GAME_COMPONENT as *mut usize) + 15032;
                println!("evts_b {:?}", read_segm(evts_b, 256));
                println!("");
                println!("evts_e {:?}", read_segm(evts_e, 256));
            }
            "test" => {
                let game = &**(P_MAIN_GAME_COMPONENT as *mut *mut CMainGameComponent);
                println!("main_game_component c_game_component quit {}", game.c_game_component.quit);
                println!("main_game_component c_game_component running {}", game.c_game_component.running);
                println!("main_game_component force_update_tick {}", game.force_update_tick);
                println!("main_game_component force_update_tick_speed_multiplier {}", game.force_update_tick_speed_multiplier);
                println!("main_game_component force_update_tick_speed_desired_framerate {}", game.force_update_tick_speed_desired_framerate);
                println!("main_game_component force_update_no_failed_updates {}", game.force_update_no_failed_updates);
                println!("main_game_component first_world_frame_update {}", game.first_world_frame_update);
                println!("main_game_component current_server_frame {}", game.current_server_frame);
                println!("main_game_component input_server_frame {}", game.input_server_frame);
                println!("main_game_component last_game_turn_force_rendered {}", game.last_game_turn_force_rendered);
                println!("main_game_component current_frame_start_game_time {}", game.current_frame_start_game_time);
                println!("main_game_component game_start_time {}", game.game_start_time);
                println!("main_game_component last_frame_render_duration {}", game.last_frame_render_duration);
                println!("main_game_component no_render_frames_since_last_game_update {}", game.no_render_frames_since_last_game_update);
                println!("main_game_component world_seed {}", game.world_seed);
                println!("main_game_component local_seed {}", game.local_seed);
                println!("main_game_component loading_event_packages {}", game.loading_event_packages);
                println!("main_game_component saving_event_packages {}", game.saving_event_packages);
                println!("main_game_component last_render_frame_start_time {}", game.last_render_frame_start_time);
                println!("main_game_component time_passed_since_last_update {}", game.time_passed_since_last_update);
                println!("main_game_component last_update_time {}", game.last_update_time);
                println!("main_game_component world_update_turn {}", game.world_update_turn);
                println!("main_game_component initialised {}", game.initialised);
                println!("main_game_component allow_render {}", game.allow_render);
                println!("main_game_component rendered {}", game.rendered);
            },
            "set_framerate_120" => {
                let main_game_component_ptr = P_MAIN_GAME_COMPONENT as *mut *mut CMainGameComponent;
                let main_game_component = *main_game_component_ptr;
                (*main_game_component).force_update_tick_speed_desired_framerate = 120.0;
            }
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

unsafe fn read_segm<'a>(address: usize, length: usize) -> &'a [[u8; 4]] {
    std::slice::from_raw_parts(address as *mut [u8; 4], length)
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