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
    let mut stdout = std::io::stdout();

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        print!("> ");

        stdout.flush().unwrap();

        let line = lines.next().unwrap().unwrap();

        match line.as_ref() {
            "" => println!("No command given."),
            "test_player" => {
                let game = &mut **(P_MAIN_GAME_COMPONENT as *mut *mut CMainGameComponent);

                let player_manager = &mut *game.p_player_manager;
                let players = player_manager.players.as_slice();
                let p_player = players[player_manager.main_player as usize];
                let mut player = &mut *players[player_manager.main_player as usize];

                let player_data = std::slice::from_raw_parts(p_player as *mut u8, 8000);
                hex_table::HexTable::new(16, p_player as usize, false, true, false).format(player_data, &mut stdout).unwrap();

                // player.show_world_thing = !player.using_free_cam;
                // player.using_free_cam = !player.using_free_cam;
                // player.controlling_free_camera = !player.controlling_free_camera;
                // player.kill_everything_mode = !player.kill_everything_mode;

                // player.drawing_free_cam_debug = true;

                // println!("player interface {:#?} {:#?}", game.p_player_interface.0, player.player_interface);
                // println!("player manager {:#?} {:#?}", game.p_player_manager.0, player.player_manager);
                // println!("world {:#?} {:#?}", game.p_world.0, player.world);

                // println!("player {:#?}", player);
            },
            "test_world" => {
                let game = &mut **(P_MAIN_GAME_COMPONENT as *mut *mut CMainGameComponent);

                let world_data = std::slice::from_raw_parts(game.p_world.0 as *mut u8, 8000);
                hex_table::HexTable::default().format(world_data, &mut stdout).unwrap();

                let mut world = &mut *game.p_world;

                // let mut file = std::fs::OpenOptions::new().write(true).create(true).open("C:\\Users\\jamen\\Documents\\Fable Resources\\CWorld.txt").unwrap();
                // write!(file, "{:#?}", world).unwrap();

                let player_manager = &mut *game.p_player_manager;
                let players = player_manager.players.as_slice();
                let p_player = players[player_manager.main_player as usize];
                let player = &mut *players[player_manager.main_player as usize];

                world.show_debug_text = !world.show_debug_text;
                world.slow_motion = !world.slow_motion;
                world.show_profile_text = !world.show_profile_text;
                world.mini_map_enabled = !world.mini_map_enabled;
                world.hero_sleeping_enabled = !world.hero_sleeping_enabled;

                // println!("game.p_world.0 {:#?}", game.p_world.0);
                // println!("player.world {:#?}", player.world);
                // println!("");
                // println!("game.p_player_manager.0 {:#?}", game.p_player_manager.0);
                // println!("world.player_manager {:#?}", world.player_manager);
                // println!("");
                // println!("main_game_component {:#?}", *(P_MAIN_GAME_COMPONENT as *mut *mut ()));
                // println!("world.component {:#?}", world.component);

                // println!("{:#?}", world);
                // println!("c_base_class_non_copyable {:#?}", world.c_base_class_non_copyable);
                // println!("ci_draw_world {:#?}", world.ci_draw_world);
                // println!("component {:#?}", world.component);
                // println!("player_manager {:#?}", world.player_manager);
                // println!("definition_manager {:#?}", world.definition_manager);
                // println!("p_world_map {:#?}", world.p_world_map);
                // println!("p_environment {:#?}", world.p_environment);
                // println!("p_game_time_manager {:#?}", world.p_game_time_manager);
                // println!("p_thing_search_tools {:#?}", world.p_thing_search_tools);
                // println!("p_atmos_processor {:#?}", world.p_atmos_processor);
                // println!("p_game_camera {:#?}", world.p_game_camera);
                // println!("p_game_camera_manager {:#?}", world.p_game_camera_manager);
                // println!("p_current_game_camera {:#?}", world.p_current_game_camera);
                // println!("p_game_script_interface {:#?}", world.p_game_script_interface);
                // println!("p_main_mesh_bank {:#?}", world.p_main_mesh_bank);
                // println!("p_animation_manager {:#?}", world.p_animation_manager);
                // println!("p_navigation_manager {:#?}", world.p_navigation_manager);
                // println!("p_thing_combat_manager {:#?}", world.p_thing_combat_manager);
                // println!("p_thing_manager {:#?}", world.p_thing_manager);
                // println!("p_faction_manager {:#?}", world.p_faction_manager);
                // println!("p_script_info_manager {:#?}", world.p_script_info_manager);
                // println!("p_message_event_manager {:#?}", world.p_message_event_manager);
                // println!("p_bullet_time_manager {:#?}", world.p_bullet_time_manager);
                // println!("p_music_manager {:#?}", world.p_music_manager);
                // println!("p_opinion_reaction_manager {:#?}", world.p_opinion_reaction_manager);
                // println!("p_script_conversation_manager {:#?}", world.p_script_conversation_manager);
                // println!("just_loaded {:#?}", world.just_loaded);
                // println!("current_world_name {:#?}", world.current_world_name);
                // println!("console_pause_at_frame_number {:#?}", world.console_pause_at_frame_number);
                // println!("frame_started_3d_rendering {:#?}", world.frame_started_3d_rendering);
                // println!("last_update_time_length {:#?}", world.last_update_time_length);
                // println!("last_update_time {:#?}", world.last_update_time);
                // println!("countdown_timer {:#?}", world.countdown_timer);
                // println!("paused {:#?}", world.paused);
                // println!("slow_motion {:#?}", world.slow_motion);
                // println!("show_debug_text {:#?}", world.show_debug_text);
                // println!("show_fps_text {:#?}", world.show_fps_text);
                // println!("show_profile_text {:#?}", world.show_profile_text);
                // println!("creature_generation_disabled_groups {:#?}", world.creature_generation_disabled_groups);
                // println!("creature_generation_enabled {:#?}", world.creature_generation_enabled);
                // println!("teleporting_enabled {:#?}", world.teleporting_enabled);
                // println!("experience_spending_enabled {:#?}", world.experience_spending_enabled);
                // println!("saving_enabled {:#?}", world.saving_enabled);
                // println!("dont_populate_next_loaded_region {:#?}", world.dont_populate_next_loaded_region);
                // println!("hero_sleeping_enabled {:#?}", world.hero_sleeping_enabled);
                // println!("map_table_show_quest_cards_on_used {:#?}", world.map_table_show_quest_cards_on_used);
                // println!("screen_to_fade_in_on_next_region_change {:#?}", world.screen_to_fade_in_on_next_region_change);
                // println!("done_extra_frame_update_before_region_load_screen_fade_in {:#?}", world.done_extra_frame_update_before_region_load_screen_fade_in);
                // println!("mini_map_enabled {:#?}", world.mini_map_enabled);
                // println!("mini_map_active_before_disabled {:#?}", world.mini_map_active_before_disabled);
            },
            "test_display" => {
                let game = &**(P_MAIN_GAME_COMPONENT as *mut *mut CMainGameComponent);
                let mut display = &mut *game.p_display_engine.0;

                // display_engine.draw_game = false;
                display.draw_memory_use = true;
                display.draw_debug_page = 2;

                println!("{:#?}", display);
            },
            "test_script" => {
                let game = &mut **(P_MAIN_GAME_COMPONENT as *mut *mut CMainGameComponent);
                let world = &mut *game.p_world;
                let script_interface = &mut *world.p_game_script_interface;

                // let script_data = std::slice::from_raw_parts(script_interface.vmt as *mut u32, 8000);
                // println!("{:x?}", script_data);
                // hex_table::HexTable::default().format(script_data, &mut stdout).unwrap();

                // let script_fns = &*script_interface.vmt;

                // println!("script_fns {:#?}", script_fns);

                // println!("is xbox {}", (script_fns.is_xbox)(world.p_game_script_interface.0));

                // (script_fns.deactivate_boast_ui)(world.p_game_script_interface.0);
                // script_interface.deactivate_boast_ui();
                // script_interface.start_sneaking();
                // script_interface.set_hero_weapons_as_usable(true);
                // script_interface.give_hero_gold(1000);
                // script_interface.set_hud_enabled(false);
                // script_interface.hero_play_fireheart_minigame();
                // script_interface.return_all_confiscated_items_to_hero();
                // script_interface.set_hero_weapons_as_usable(true);
                // script_interface.set_hero_will_as_usable(true);
                // script_interface.set_weapon_out_crime_enabled(false);
                // script_interface.remove_all_hero_weapons();

                // let hero = &script_interface.get_hero();
                // script_interface.entity_set_max_walking_speed(hero, 100.0);
                // script_interface.set_debug_camera_type(1);

                // script_interface.entity_set_max_walking_speed(&script_interface.hero_script_thing as *const tlse_sys::CScriptThing, 100.0);

                // let t = script_interface.text_entry_exists(&tlse_sys::CCharString::new("SWORD_OF_AEONS".to_string()) as *const tlse_sys::CCharString);
                // println!("text_entry_exists {}", t);

                script_interface.create_creature_nearby(
                    &tlse_sys::CCharString::new("CREATURE_BALVERINE".to_string()) as *const tlse_sys::CCharString,
                    &tlse_sys::C3DVector { x: 0.0, y: 0.0, z: 0.0 } as *const tlse_sys::C3DVector,
                );
            },
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
                println!("main_game_component no_render_frames_since_last_game_update {}", game.render_frames_since_last_game_update_count);
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

