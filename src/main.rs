use bevy::prelude::*;

mod constants;
mod debug;

use constants::*;
use debug::DebugPlugin;

fn main() {
    App::new()
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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(DebugPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        })
        .insert(Name::new("Portal"));

    // cube
    let ground_model = Mat4::from_scale_rotation_translation(
        Vec3::new(1000.0, 1.0, 1000.0),
        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
        portal_position,
    );

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            transform: Transform::from_matrix(ground_model),
            ..Default::default()
        })
        .insert(Name::new("Ground"));

    // light
    let light_model = Mat4::from_scale_rotation_translation(
        Vec3::splat(5.0),
        Quat::from_euler(EulerRot::XYZ, 0.0, 2.4, 0.0),
        portal_position + Vec3::Y,
    );

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform::from_matrix(light_model),
        ..Default::default()
    });

    spawn_tree(
        &mut commands,
        &asset_server,
        Vec3::new(421.610901, 256.906128, 694.026062),
    );
    spawn_tree(
        &mut commands,
        &asset_server,
        Vec3::new(445.025879, 256.906128, 726.467102),
    );
    spawn_tree(
        &mut commands,
        &asset_server,
        Vec3::new(442.042084, 256.906128, 700.193604),
    );
    spawn_tree(
        &mut commands,
        &asset_server,
        Vec3::new(457.402618, 256.906128, 719.416687),
    );
    spawn_tree(
        &mut commands,
        &asset_server,
        Vec3::new(433.07251, 256.906128, 738.293701),
    );
    spawn_tree(
        &mut commands,
        &asset_server,
        Vec3::new(408.839081, 256.906128, 748.451233),
    );
    spawn_tree(
        &mut commands,
        &asset_server,
        Vec3::new(397.201599, 256.906128, 726.434509),
    );
}

fn spawn_tree(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3) {
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
        .insert(Name::new("Tree"));
}
