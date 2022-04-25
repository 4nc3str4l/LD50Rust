use bevy::prelude::*;

use crate::{components::*, constants::*, spawners::*, GameState};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub struct StartScenePlugin;
impl Plugin for StartScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::StartScene)
            .add_system_set(
                SystemSet::on_enter(GameState::StartScene)
                    .with_system(setup_start_scene)
                    .with_system(setup_ui),
            )
            .add_system_set(
                SystemSet::on_update(GameState::StartScene)
                    .with_system(update_start_scene)
                    .with_system(button_system),
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
        Vec3::splat(3.0),
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

    spawn_rotating_soul(
        &mut commands,
        &asset_server,
        portal_position,
        10.0,
        10.0,
        0.0,
        2.0,
        false,
        false,
    );
    spawn_rotating_soul(
        &mut commands,
        &asset_server,
        portal_position,
        25.0,
        25.0,
        0.0,
        1.5,
        true,
        false,
    );
    spawn_rotating_soul(
        &mut commands,
        &asset_server,
        portal_position,
        45.0,
        45.0,
        2.0,
        0.5,
        false,
        false,
    );
    spawn_rotating_soul(
        &mut commands,
        &asset_server,
        portal_position,
        5.0,
        4.0,
        0.0,
        2.0,
        false,
        true,
    );
    spawn_rotating_soul(
        &mut commands,
        &asset_server,
        portal_position,
        8.0,
        8.0,
        0.0,
        0.3,
        false,
        true,
    );
    spawn_rotating_soul(
        &mut commands,
        &asset_server,
        portal_position,
        6.0,
        6.0,
        0.0,
        1.0,
        true,
        true,
    );
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),

            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: Rect {
                        top: Val::Px(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Soul Dilemma",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Start",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}

pub fn update_start_scene(mut trees: Query<(&Tree, &mut Transform)>, time: Res<Time>) {
    // for (tree, mut transform) in trees.iter_mut() {
    //     transform.translation.x += 1.0 * time.delta_seconds();
    // }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                println!("Hello World!");
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn dispose_start_scene() {}
