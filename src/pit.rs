use bevy::{math::*, prelude::*};

const PIT_TRANSLATION: Vec3 = Vec3::new(700.0, 400.0, 0.0);
use crate::{
    assetloader::Textures,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    space::AddSpatialEntity,
};

#[derive(Resource)]
pub struct TribeStoneQue {
    pub value: Vec<Vec3>,
}

#[derive(Component)]
pub struct Pit;

#[derive(Resource)]
pub struct TribeGoal {
    pub value: String,
}

pub struct PitPlugin;

impl Plugin for PitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TribeGoal {
            value: "GET STONES".to_string(),
        })
        .insert_resource(TribeStoneQue { value: vec![] })
        .add_systems(Startup, setup)
        .add_systems(Update, pit_logic_function);
    }
}

fn pit_logic_function(
    mut commands: Commands,
    mut space_update_receiver: EventReader<AddSpatialEntity>,
    mut stone_que: ResMut<TribeStoneQue>,
) {
    for event in space_update_receiver.read() {
        info!("{}", event.value);
        stone_que.value.push(event.value);
        info!("{}", stone_que.value[0]);
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
