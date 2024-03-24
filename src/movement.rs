use bevy::prelude::*;

use crate::{
    minions::{Minion, MinionInstructionSet},
    mouse::WorldLastClicked,
    pit::Pit,
    space::{map_vec3_hash, SpaceMap},
    stone::Stone,
    MinionUpdateSet,
};

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
        app.add_systems(
            Update,
            ((update_velocity, update_position).chain())
                .in_set(MinionUpdateSet::PostDecisionMaking),
        );
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn update_position(
    mut minion_query: Query<
        (&Velocity, &mut Transform, &Minion, &MinionInstructionSet),
        (With<Minion>, Without<Stone>),
    >,
    mut stone_query: Query<&Transform, (Without<Minion>, With<Stone>)>,
    time: Res<Time>,
    last_clicked_world_coordinates: Res<WorldLastClicked>,
    space_map: Res<SpaceMap>,
) {
    //space_map.value[0];
    //    let index = (transform.translation.x as usize) * 1920 + (transform.translation.y as usize);

    for (velocity, mut transform, minion, mis) in minion_query.iter_mut() {
        /*  println!(
            "{:?}",
            space_map.value.get(&map_vec3_hash(&transform.translation))
        ); */
        match mis.target {
            None => continue,
            Some(val) => {
                let pos = transform.translation;

                transform.translation +=
                    velocity.value * get_direction(pos, val) * time.delta_seconds();
            }
        }
    }
}

fn get_direction(position: Vec3, target: Vec3) -> Vec3 {
    Vec3::new(
        (position.x - target.x) * -1.0,
        (position.y - target.y) * -1.0,
        0.0,
    )
}
