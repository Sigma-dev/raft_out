// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

use crate::{raft_out::RaftOutPlugin, text_renderer::TextRendererPlugin};

mod direction;
mod raft_out;
mod text_renderer;

fn main() -> AppExit {
    App::new()
        .add_plugins(TextRendererPlugin)
        .add_plugins(RaftOutPlugin)
        .run()
}
