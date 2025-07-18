// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

use crate::{
    audio_manager::AudioManagerPlugin, raft_out::RaftOutPlugin, text_renderer::TextRendererPlugin,
};

mod audio_manager;
mod direction;
mod raft_out;
mod text_renderer;

fn main() -> AppExit {
    App::new()
        .add_plugins(TextRendererPlugin)
        .add_plugins(RaftOutPlugin)
        .add_plugins(AudioManagerPlugin { volume_mult: 0.1 })
        .run()
}
