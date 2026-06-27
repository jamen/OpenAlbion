//! Lowering: convert parsed text [`Definition`]s into the typed binary def
//! structs from `fable-data`.
//!
//! Each `lower_*` function drives a [`DefReader`] over a definition body and
//! produces the corresponding def struct. [`lower_def`] dispatches by def name
//! (using the same name strings as the binary `DefBody::parse`).

use fable_data::def::binary::def_binary::DefBody;
use fable_data::def::text::{Definition, SymbolTable};
use fable_data::def::{
    ConfigOptionsDefaultsDef, EngineDef, EngineVideoOptionsDef, FrontEndDef, UiIconsDef,
};

use crate::reader::{DefReader, DefReaderError};

#[derive(Debug)]
pub enum LowerError {
    /// No lowering is implemented for this def type yet.
    Unsupported(String),
    Reader(DefReaderError),
}

impl From<DefReaderError> for LowerError {
    fn from(error: DefReaderError) -> Self {
        LowerError::Reader(error)
    }
}

/// Lower a definition into a [`DefBody`], dispatching on `name` (the def type,
/// matching the binary naming used in `DefBody::parse`).
pub fn lower_def(
    name: &str,
    def: &Definition,
    symbols: &SymbolTable,
) -> Result<DefBody, LowerError> {
    Ok(match name {
        "ENGINE" => DefBody::Engine(lower_engine(def, symbols)?),
        "FRONT_END" => DefBody::FrontEnd(lower_front_end(def, symbols)?),
        "UI_ICONS_DEF" => DefBody::UiIcons(lower_ui_icons(def, symbols)?),
        "ENGINE_VIDEO_OPTIONS" => DefBody::EngineVideoOptions(lower_engine_video_options(def, symbols)?),
        "CONFIG_OPTIONS_DEFAULTS_DEF" => {
            DefBody::ConfigOptionsDefaults(lower_config_options_defaults(def, symbols)?)
        }
        _ => return Err(LowerError::Unsupported(name.to_string())),
    })
}

pub fn lower_engine(def: &Definition, symbols: &SymbolTable) -> Result<EngineDef, DefReaderError> {
    let mut f = DefReader::new(&def.body, symbols);

    let out = EngineDef {
        lod_error_tolerance: f.f32("LODErrorTolerance")?,
        character_lod_error_tolerance: f.f32("CharacterLODErrorTolerance")?,
        lod_error_factor: f.f32("LODErrorFactor")?,
        sea_height: f.f32("SeaHeight")?,
        local_detail_boolean_alpha_default_alpha_ref: f
            .i32("LocalDetailBooleanAlphaDefaultAlphaRef")?,
        default_primitive_alpha_ref: f.i32("DefaultPrimitiveAlphaRef")?,
        game_primitive_default_fade_start: f.f32("GamePrimitiveDefaultFadeStart")?,
        game_primitive_default_fade_range_ratio: f.f32("GamePrimitiveDefaultFadeRangeRatio")?,
        local_detail_default_fade_start: f.f32("LocalDetailDefaultFadeStart")?,
        local_detail_default_fade_range_ratio: f.f32("LocalDetailDefaultFadeRangeRatio")?,
        test_static_mesh: f.i32("TestStaticMesh")?,
        test_animated_mesh: f.i32("TestAnimatedMesh")?,
        test_anim: f.i32("TestAnim")?,
        test_graphic: f.i32("TestGraphic")?,
        fov_2d: f.f32("FOV_2D")?,
        invalid_texture_standin: f.i32("InvalidTextureStandin")?,
        invalid_theme_standin: f.i32("InvalidThemeStandin")?,
    };

    f.finish()?;

    Ok(out)
}

pub fn lower_config_options_defaults(
    def: &Definition,
    symbols: &SymbolTable,
) -> Result<ConfigOptionsDefaultsDef, DefReaderError> {
    let mut f = DefReader::new(&def.body, symbols);

    let out = ConfigOptionsDefaultsDef {
        antialiasing: f.i32("Antialiasing")?,
        resolution_width: f.u32("ResolutionWidth")?,
        resolution_height: f.u32("ResolutionHeight")?,
        bit_depth: f.u32("BitDepth")?,
        texture_detail: f.f32("TextureDetail")?,
        max_texture_detail: f.f32("MaxTextureDetail")?,
        shadow_detail: f.f32("ShadowDetail")?,
        max_shadow_detail: f.f32("MaxShadowDetail")?,
        mesh_detail: f.f32("MeshDetail")?,
        max_mesh_detail: f.f32("MaxMeshDetail")?,
        effects_detail: f.f32("EffectsDetail")?,
        max_effects_detail: f.f32("MaxEffectsDetail")?,
        min_resolution_width: f.i32("MinResolutionWidth")?,
        min_resolution_height: f.i32("MinResolutionHeight")?,
    };

    f.finish()?;

    Ok(out)
}

pub fn lower_engine_video_options(
    def: &Definition,
    symbols: &SymbolTable,
) -> Result<EngineVideoOptionsDef, DefReaderError> {
    let mut f = DefReader::new(&def.body, symbols);

    let out = EngineVideoOptionsDef {
        hires_texture_memory: f.i32("HiresTextureMemory")?,
        lod_error_tolerance: f.f32("LODErrorTolerance")?,
        character_lod_error_tolerance: f.f32("CharacterLODErrorTolerance")?,
        draw_distance_multiplier: f.f32("DrawDistanceMultiplier")?,
        draw_distance_minimum: f.f32("DrawDistanceMinimum")?,
        draw_distance_maximum: f.f32("DrawDistanceMaximum")?,
        repeated_mesh_draw_distance_factor: f.f32("RepeatedMeshDrawDistanceFactor")?,
        minimum_z_sprite_as_mesh_distance: f.f32("MinimumZSpriteAsMeshDistance")?,
        maximum_z_sprite_as_mesh_distance: f.f32("MaximumZSpriteAsMeshDistance")?,
        z_sprite_draw_distance_multiplier: f.f32("ZSpriteDrawDistanceMultiplier")?,
        shadow_buffer_size: f.i32("ShadowBufferSize")?,
        shadow_distance_scale: f.f32("ShadowDistanceScale")?,
        enable_2d_displacement: f.bool("Enable2DDisplacement")?,
        enable_3d_displacement: f.bool("Enable3DDisplacement")?,
        enable_glow: f.bool("EnableGlow")?,
        enable_radial_blur: f.bool("EnableRadialBlur")?,
        enable_water_reflection: f.bool("EnableWaterReflection")?,
        enable_weather_effects: f.bool("EnableWeatherEffects")?,
        enable_colour_filter: f.bool("EnableColourFilter")?,
        weather_density: f.f32("WeatherDensity")?,
        enable_repeated_meshes: f.bool("EnableRepeatedMeshes")?,
    };

    f.finish()?;

    Ok(out)
}

pub fn lower_ui_icons(
    def: &Definition,
    symbols: &SymbolTable,
) -> Result<UiIconsDef, DefReaderError> {
    let mut f = DefReader::new(&def.body, symbols);

    let out = UiIconsDef {
        icon_friend_request_received: f.u32("IconFriendRequestReceived")?,
        icon_friend_request_received_on: f.u32("IconFriendRequestReceivedOn")?,
        icon_friend_request_sent: f.u32("IconFriendRequestSent")?,
        icon_friend_request_sent_on: f.u32("IconFriendRequestSentOn")?,
        icon_game_invite_received: f.u32("IconGameInviteReceived")?,
        icon_game_invite_received_on: f.u32("IconGameInviteReceivedOn")?,
        icon_game_invite_sent: f.u32("IconGameInviteSent")?,
        icon_game_invite_sent_on: f.u32("IconGameInviteSentOn")?,
        icon_mute: f.u32("IconMute")?,
        icon_mute_on: f.u32("IconMuteOn")?,
        icon_online: f.u32("IconOnline")?,
        icon_online_on: f.u32("IconOnlineOn")?,
        icon_passcode_blank: f.u32("IconPasscodeBlank")?,
        icon_passcode_filled: f.u32("IconPasscodeFilled")?,
        icon_tv: f.u32("IconTV")?,
        icon_tv_on: f.u32("IconTVOn")?,
        icon_voice: f.u32("IconVoice")?,
        icon_voice_on: f.u32("IconVoiceOn")?,
        icon_wait_1: f.u32("IconWait1")?,
        icon_wait_2: f.u32("IconWait2")?,
        icon_wait_3: f.u32("IconWait3")?,
        icon_wait_4: f.u32("IconWait4")?,
        icon_progress: f.u32("IconProgress")?,
        icon_progress_on: f.u32("IconProgressOn")?,
        icon_a: f.u32("IconA")?,
        icon_b: f.u32("IconB")?,
        icon_x: f.u32("IconX")?,
        icon_y: f.u32("IconY")?,
        icon_blank: f.u32("IconBlank")?,
        icon_up_arrow: f.u32("IconUpArrow")?,
        icon_down_arrow: f.u32("IconDownArrow")?,
        icon_list_highlight: f.u32("IconListHighlight")?,
    };

    f.finish()?;

    Ok(out)
}

pub fn lower_front_end(
    def: &Definition,
    symbols: &SymbolTable,
) -> Result<FrontEndDef, DefReaderError> {
    let mut r = DefReader::new(&def.body, symbols);

    let attract_mode_movie: Vec<String> = r
        .indexed("vAttractModeMovie")?
        .into_iter()
        .map(|mut elem| elem.any_string())
        .collect::<Result<_, _>>()?;

    let out = FrontEndDef {
        attract_mode_movie,
        error_message_background_graphic: r.u32("ErrorMessageBackgroundGraphic")?,
        button_a_big_graphic: r.u32("ButtonABigGraphic")?,
        button_b_big_graphic: r.u32("ButtonBBigGraphic")?,
    };

    r.finish()?;

    Ok(out)
}
