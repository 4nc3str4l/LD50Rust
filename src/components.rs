use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Tree {
    pub size: f32,
}

#[derive(Component, Inspectable)]
pub struct Soul {}

#[derive(Component, Inspectable)]
pub struct Orbit {
    pub x_spread: f32,
    pub z_spread: f32,
    pub y_offset: f32,
    pub center: Vec3,

    pub rot_speed: f32,
    pub clock_wise: bool,

    pub timer: f32,
}
