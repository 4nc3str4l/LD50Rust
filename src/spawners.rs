use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

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
        .insert(Soul {});
}

pub fn spawn_rotating_soul(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    x_spread: f32,
    z_spread: f32,
    y_offset: f32,
    rot_speed: f32,
    clock_wise: bool,
    is_red: bool,
) {
    let (glft_model, scale, name, light_color) = match is_red {
        true => (
            asset_server.load(MODEL_RED_SOUL),
            0.5,
            "Red Soul",
            Color::RED,
        ),
        false => (
            asset_server.load(MODEL_BLUE_SOUL),
            1.0,
            "Blue Soul",
            Color::rgb(0.0, 187.0 / 255.0, 231.0 / 255.0),
        ),
    };

    // Spawn a second scene, and keep its `instance_id`
    let soul_model = Mat4::from_scale_rotation_translation(
        Vec3::splat(scale),
        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
        position,
    );

    commands
        .spawn_bundle((
            Transform::from_matrix(soul_model),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(glft_model);
            parent.spawn_bundle(PointLightBundle {
                point_light: PointLight {
                    intensity: 500.0,
                    shadows_enabled: false,
                    color: light_color,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..Default::default()
            });
        })
        .insert(Name::new(name))
        .insert(Soul {})
        .insert(Orbit {
            x_spread: x_spread,
            z_spread: z_spread,
            y_offset: y_offset,
            center: position,
            rot_speed: rot_speed,
            clock_wise: clock_wise,
            timer: 0.0,
        });
}

pub fn spawn_camera(commands: &mut Commands) {
    // Spawn camera
    let camera_model = Mat4::from_scale_rotation_translation(
        Vec3::splat(1.0),
        Quat::from_euler(EulerRot::XYZ, -2.5, -0.5, -3.0),
        Vec3::new(386.5, 286.0, 681.0),
    );

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(camera_model),
            ..Default::default()
        })
        .insert(Name::new("Camera"));
}
