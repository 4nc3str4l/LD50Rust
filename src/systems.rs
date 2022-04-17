use bevy::prelude::*;

use crate::components::*;

pub fn sys_orbit(mut orbits: Query<(&mut Orbit, &mut Transform)>, time: Res<Time>) {
    for (mut orbit, mut transform) in orbits.iter_mut() {
        orbit.timer += time.delta_seconds() * orbit.rot_speed;
        let mut rot_dir = 1.0;
        if orbit.clock_wise {
            rot_dir = -1.0;
        }
        let x = rot_dir * orbit.timer.cos() * orbit.x_spread;
        let z = orbit.timer.sin() * orbit.z_spread;
        transform.translation.x = orbit.center.x + x;
        transform.translation.y = orbit.center.y + orbit.y_offset;
        transform.translation.z = orbit.center.z + z;
    }
}
