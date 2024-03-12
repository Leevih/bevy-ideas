use bevy::prelude::*;

use crate::{mouse::WorldLastClicked, pit::Pit};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SpriteBundle,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_position).chain());
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn update_position(
    mut query: Query<(&Velocity, &mut Transform), Without<Pit>>,
    time: Res<Time>,
    last_clicked_world_coordinates: Res<WorldLastClicked>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        let pos = transform.translation;
        transform.translation += velocity.value
            * get_direction(pos, last_clicked_world_coordinates.value)
            * time.delta_seconds();
    }
}

fn get_direction(position: Vec3, target: Vec3) -> Vec3 {
    Vec3::new(
        (position.x - target.x) * -1.0,
        (position.y - target.y) * -1.0,
        0.0,
    )
}
