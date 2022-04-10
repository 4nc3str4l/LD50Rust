use bevy::prelude::*;

mod debug;

use debug::DebugPlugin;

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
    let camera_model = Mat4::from_scale_rotation_translation(
        Vec3::splat(1.0),
        Quat::from_euler(EulerRot::XYZ, 31.8346062, 29.6680508, -2.00988575e-06),
        Vec3::new(405.748962, 278.687042, 686.857544),
    );

    let bundle = commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(camera_model)
            .looking_at(Vec3::new(409.8714, 256.4061, 721.0326), Vec3::Y),
        ..Default::default()
    });

    let mut glft_model = asset_server.load("models/portal.gltf#Scene0");

    // Spawn a second scene, and keep its `instance_id`
    let portal_model = Mat4::from_scale_rotation_translation(
        Vec3::splat(5.0),
        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
        Vec3::new(409.8714, 256.4061, 721.0326),
    );

    commands
        .spawn_bundle((
            Transform::from_matrix(portal_model),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(glft_model);
        });
}
