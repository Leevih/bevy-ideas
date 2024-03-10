use bevy::prelude::*;
use crate::mouse::MyWorldCoords;

#[derive(Component)]
struct Minion {
    velocity: Vec2,
}

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_minions).add_systems(Update, minion_position_system);
    }
}

fn spawn_minions(mut commands: Commands) {
    commands.spawn(Minion {
        velocity: Vec2::new(0.0, 0.0)
    });
}

fn minion_position_system(
    mycoords:Res<MyWorldCoords>,
){
    info!("{}", mycoords.value);

    // get mouse from mouse system
    // Create new pos from entity pos / mouse pos / velocity
    // apply new Transform::translation
}