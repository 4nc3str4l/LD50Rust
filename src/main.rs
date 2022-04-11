use bevy::{prelude::*, window::WindowMode};

mod components;
mod constants;
mod debug;
mod spawners;
mod start_scene;
mod systems;

use constants::*;
use debug::DebugPlugin;
use start_scene::*;
use systems::sys_orbit;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    StartScene,
    ExplanationScene,
    GameScene,
    DayState,
    GameOverScene,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: WINDOW_TITLE.to_string(),
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_plugin(DebugPlugin)
        .add_plugin(StartScenePlugin)
        .add_system(sys_orbit)
        .run();
}
