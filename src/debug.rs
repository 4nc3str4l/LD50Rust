use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::components::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            println!("Debug Mode");
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Tree>()
                .register_inspectable::<Soul>()
                .register_inspectable::<Orbit>();
        }
    }
}
