use bevy::prelude::*;

mod debug;
mod constants;

use debug::DebugPlugin;
use constants::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(DebugPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    
    // Spawn camera
    let camera_model = Mat4::from_scale_rotation_translation(
        Vec3::splat(1.0),
        Quat::from_euler(EulerRot::XYZ, -2.564, -0.101, -3.076),
        Vec3::new(386.32, 300.2, 660.5),
    );

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(camera_model),
        ..Default::default()
    });

    // Spawn portal
    let portal_position = Vec3::new(409.8714, 256.4061, 721.0326);
    let glft_model = asset_server.load(MODEL_PORTAL);

    // Spawn a second scene, and keep its `instance_id`
    let portal_model = Mat4::from_scale_rotation_translation(
        Vec3::splat(5.0),
        Quat::from_euler(EulerRot::XYZ, 0.0, 2.4, 0.0),
        portal_position,
    );

    commands
        .spawn_bundle((
            Transform::from_matrix(portal_model),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(glft_model);
        }).insert(Name::new("Portal"));
    
}
