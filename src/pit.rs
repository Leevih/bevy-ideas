use bevy::{math::*, prelude::*};

const PIT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
use crate::{
    assetloader::Textures,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

#[derive(Component)]
pub struct Pit;

pub struct PitPlugin;

impl Plugin for PitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, textures: Res<Textures>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::new(0.0, 0.0, 0.0)),
            acceleration: Acceleration::new(Vec3::new(0.0, 0.0, 0.0)),
            model: SpriteBundle {
                transform: Transform {
                    translation: PIT_TRANSLATION,
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgba(0.5, 0.5, 0.5, 1.0),
                    custom_size: Some(Vec2::new(80.0, 80.0)),
                    ..Default::default()
                },
                texture: textures.pit.clone(),
                ..Default::default()
            },
        },
        Pit,
    ));
}
