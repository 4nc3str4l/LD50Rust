use bevy::prelude::*;

use crate::{components::*, constants::*, spawners::*, GameState};

pub struct StartScenePlugin;
impl Plugin for StartScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::StartScene)
            .add_system_set(
                SystemSet::on_enter(GameState::StartScene).with_system(setup_start_scene),
            )
            .add_system_set(
                SystemSet::on_update(GameState::StartScene).with_system(update_start_scene),
            );
    }
}

pub fn setup_start_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_camera(&mut commands);

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

    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: false,
                color: Color::RED,
                ..Default::default()
            },
            transform: Transform::from_matrix(light_model),
            ..Default::default()
        })
        .insert(Name::new("Portal Light"));

    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: false,
                illuminance: 1000.0,
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform::from_matrix(light_model),
            ..Default::default()
        })
        .insert(Name::new("Sun"));

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

pub fn update_start_scene(mut trees: Query<(&Tree, &mut Transform)>, time: Res<Time>) {
    // for (tree, mut transform) in trees.iter_mut() {
    //     transform.translation.x += 1.0 * time.delta_seconds();
    // }
}

pub fn dispose_start_scene() {}
