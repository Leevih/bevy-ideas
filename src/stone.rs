use bevy::{math, prelude::*, reflect::Array, utils::HashMap};

use crate::{
    assetloader::Textures,
    mouse::{MouseWorldEvent, WorldLastClicked},
    movement::{Acceleration, MovingObjectBundle, Velocity},
    space::AddSpatialEntity,
};

#[derive(Component)]
pub struct Stone;

#[derive(Event)]
pub struct DestroyStone {
    entity: Entity,
}

pub struct StonePlugin;

impl Plugin for StonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_stone_sys)
            .add_event::<DestroyStone>()
            .add_systems(Update, handle_destroy_stone_event);
    }
}

fn handle_destroy_stone_event(
    mut commands: Commands,
    mut ev_destroy_stone: EventReader<DestroyStone>,
) {
    for event in ev_destroy_stone.read() {
        commands.entity(event.entity).despawn();
    }
}

fn spawn_stone_sys(
    mut commands: Commands,
    textures: Res<Textures>,
    mut ev_mouse_world: EventReader<MouseWorldEvent>,
    mut notify_spatial_entity: EventWriter<AddSpatialEntity>,
) {
    for event in ev_mouse_world.read() {
        let landing_position: Vec3 = event.value;
        let stone_id: Entity = commands
            .spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(Vec3::new(0.0, 0.0, 0.0)),
                    acceleration: Acceleration::new(Vec3::new(0.0, 0.0, 0.0)),
                    model: SpriteBundle {
                        transform: Transform {
                            translation: landing_position,
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
            ))
            .id();
        notify_spatial_entity.send(AddSpatialEntity {
            entity: stone_id,
            value: landing_position,
        });
    }
}

//x = 0
//y = 0
//width = 1080

//x = 1
//y = 1
//width = 1080
// i = x + width*y;
