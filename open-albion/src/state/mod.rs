mod camera;
mod input;

use camera::{ArcballRig, Camera};
use input::InputState;

use fable_data::Big;
// use hecs::World;
use rand::prelude::*;
use thiserror::Error;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

pub(crate) struct State {
    pub frame_start: Instant,
    pub fable_dir: PathBuf,
    pub input: InputState,
    // pub world: World,
    pub camera: Camera,
    pub arcball_rig: ArcballRig,

    // TODO: Better system for resources
    pub graphics: fable_data::Big,
    pub graphics_file: File,

    pub textures: fable_data::Big,
    pub textures_file: File,

    pub selected_model_name: String,
    pub model_vector_clock: usize,
    pub wireframe: bool,
    pub show_focus_point: bool,
}

#[derive(Error, Debug)]
pub enum StateError {
    #[error("Fable's directory not found.")]
    FableDirNotFound,
}

impl State {
    pub fn new(fable_dir: &PathBuf) -> Result<Self, StateError> {
        let fable_dir = fable_dir.to_owned();

        let graphics_path = fable_dir.join("data/graphics/graphics.big");
        let textures_path = fable_dir.join("data/graphics/pc/textures.big");

        let mut graphics_file = File::open(&graphics_path).unwrap();
        let mut textures_file = File::open(&textures_path).unwrap();

        let graphics = Big::decode_reader_with_path(&mut graphics_file, &graphics_path).unwrap();
        let textures = Big::decode_reader_with_path(&mut textures_file, &textures_path).unwrap();

        // let selected_model_name = Self::random_model_name(&graphics);
        let selected_model_name = //
            // "MESH_ARROWHEAD_FIRE"
            // "MESH_HERO_FOLDED_TROUSERS_ARMOUR_GOOD"
            // "MESH_WATERFALL_CAVERN_06"
            // "MESH_HOOKCOAST_SHOP_GENERAL_STORE_INT_01"
            // "MESH_HERO_BANDITCAMP_HEAD_01"
            // "MESH_HERO_LEATHERARMOUR_BOOT_L_01"
            // "MESH_HERO_FOLDED_BOOTS_LEATHERARMOUR"
            // "MESH_BODYGUARD_LEGS_01"
            // "MESH_PIE_APPLE_QUARTER_01"
            // "MESH_PIE_BLUEBERRY_QUARTER_01"
            "MESH_TROPHY_TOOTH"
            // "MESH_QUEST_CARD_VIN_NEUTRAL_01"
            // "MESH_CREATURE_BUTTERFLY_COMMONBLUE"
            // "MESH_CREATURE_BUTTERFLY_TORTOISESHELL"
            // "MESH_GRAVEYARD_CRYPT_SECRET_DOOR"
            // "MESH_BS_RUG_SQUARE_SCALES_01"
            // "MESH_BHF_RUG_LEVEL_03_A"
            // "MESH_POPPY_01"
            // "MESH_BS_SLUM_PLAYING_CARD_SPADES_TEN_01"
            // "MESH_GUILD_BOOKCASE_SECRET_01"
            // "MESH_GUILD_LO_POLY_01"
            // "MESH_SS_TAVERN_EXT_01"
            .to_owned();

        let camera = Camera::new();

        let arcball_rig = ArcballRig::new();

        Ok(Self {
            frame_start: Instant::now(),
            fable_dir,
            input: InputState::new(),
            camera,
            arcball_rig,
            graphics,
            graphics_file,
            textures,
            textures_file,
            selected_model_name,
            model_vector_clock: 0,
            wireframe: true,
            show_focus_point: false,
        })
    }

    pub fn handle_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        self.input.handle_event(&event);

        match event {
            Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    }

    // pub fn update(&mut self) {
    //     if self.input.cursor_position.is_some() {
    //         if self.input.modifiers.ctrl() {
    //             self.show_focus_point = true;
    //         } else if self.show_focus_point {
    //             self.show_focus_point = false;
    //         }

    //         if self.input.keys[VirtualKeyCode::Space as usize] > Some(self.frame_start) {
    //             self.selected_model_name = Self::random_model_name(&self.graphics);
    //             self.model_vector_clock += 1;
    //         }

    //         if self.input.keys[VirtualKeyCode::E as usize] > Some(self.frame_start) {
    //             self.wireframe = !self.wireframe;
    //         }
    //     }

    //     self.frame_start = Instant::now();
    // }

    pub fn random_model_name(graphics: &Big) -> String {
        graphics
            .entries_by_name
            .keys()
            .nth(rand::thread_rng().gen_range(0..graphics.entries.len()))
            .unwrap()
            .to_owned()
    }
}