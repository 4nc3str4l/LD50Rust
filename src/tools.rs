use bevy::prelude::*;

use crate::constants::*;
use crate::entities::*;

pub fn spawn_tree(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3) {
    let glft_model = asset_server.load(MODEL_TREE);

    // Spawn a second scene, and keep its `instance_id`
    let portal_model = Mat4::from_scale_rotation_translation(
        Vec3::splat(5.0),
        Quat::from_euler(EulerRot::XYZ, 0.0, 2.4, 0.0),
        position,
    );

    commands
        .spawn_bundle((
            Transform::from_matrix(portal_model),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(glft_model);
        })
        .insert(Name::new("Tree"))
        .insert(Tree { size: 1.0 });
}

pub fn spawn_camera(commands: &mut Commands) {
    // Spawn camera
    let camera_model = Mat4::from_scale_rotation_translation(
        Vec3::splat(1.0),
        Quat::from_euler(EulerRot::XYZ, -2.5, -0.5, -3.0),
        Vec3::new(390.32, 285.2, 687.5),
    );

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(camera_model),
            ..Default::default()
        })
        .insert(Name::new("Camera"));
}
