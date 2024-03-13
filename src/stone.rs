use bevy::{
    math,
    prelude::*,
    utils::{hashbrown::hash_map, HashMap},
};

use crate::{
    assetloader::Textures,
    mouse::{MouseWorldEvent, WorldLastClicked},
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

#[derive(Component)]
pub struct StoneMap {
    pub value: HashMap<Vec3, Entity>,
}

#[derive(Component)]
pub struct Stone;

pub struct StonePlugin;

impl Plugin for StonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_stone_sys);
    }
}

fn spawn_stone_sys(
    mut commands: Commands,
    last_clicked: Res<WorldLastClicked>,
    textures: Res<Textures>,
    mut ev_mouse_world: EventReader<MouseWorldEvent>,
) {
    for event in ev_mouse_world.read() {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(Vec3::new(0.0, 0.0, 0.0)),
                acceleration: Acceleration::new(Vec3::new(0.0, 0.0, 0.0)),
                model: SpriteBundle {
                    transform: Transform {
                        translation: event.value,
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::rgba(0.5, 1.0, 0.5, 1.0),
                        custom_size: Some(Vec2::new(20.0, 20.0)),
                        ..Default::default()
                    },
                    texture: textures.stone.clone(),
                    ..Default::default()
                },
            },
            Stone,
        ));
    }
}
