//! Def compilation: text [`Definition`]s → typed binary def structs.
//!
//! Depends on `fable-data` for the lower-level text parsing/symbol evaluation
//! and the binary def structs. This crate owns the [`DefReader`] (which scans a
//! definition body by field name) and the `lower_*` functions that drive it.
//!
//! [`Definition`]: fable_data::def::text::Definition

pub mod lower;
pub mod reader;

pub use self::lower::{
    LowerError, lower_config_options_defaults, lower_def, lower_engine,
    lower_engine_video_options, lower_front_end, lower_ui_icons,
};
pub use self::reader::{Args, BadIndex, DefReader, DefReaderError, EvalError, Evaluator};
