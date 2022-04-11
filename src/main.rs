use bevy::prelude::*;

mod constants;
mod debug;
mod entities;
mod start_scene;
mod tools;

use constants::*;
use debug::DebugPlugin;
use start_scene::*;

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
            width: 1280.0,
            height: 768.0,
            title: "Soul Dilemma Rust".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugin(DebugPlugin)
        .add_plugin(StartScenePlugin)
        .run();
}
