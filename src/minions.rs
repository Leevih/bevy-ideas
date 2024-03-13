use crate::assetloader::Textures;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::pit::{Pit, TribeGoal};
use bevy::{math::*, prelude::*};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Inventory {
    stone: u32,
}

#[derive(Component)]
pub struct Minion;

#[derive(Component)]
pub struct MinionInstructionSet {
    target: Option<Vec3>,
}
#[derive(Component)]

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(2.5, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_minion)
        .add_systems(Update, minion_commander);
    }
}

// Only deals with GET STONES Instructions
fn minion_commander(
    mut minion_query: Query<
        (&mut Transform, &mut Inventory, &mut MinionInstructionSet),
        With<Minion>,
    >,
    goal: Res<TribeGoal>,
) {
    if (goal.value != "GET STONES".to_string()) {
        return;
    }
    for (mut m_transform, mut m_inv, mut m_inst_set) in minion_query.iter_mut() {
        // Check inventory
        if (m_inv.stone > 0) {
            continue;
        }
        // Need to find stone.. Maybe need hash map now
    }
}

fn spawn_minion(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    textures: Res<Textures>,
    pit_query: Query<&Transform, With<Pit>>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let velocity = Vec3::new(1.0, 1.0, 0.0);
    let acceleration = Vec3::new(1.0, 1.0, 0.0);
    let pit_transform = pit_query.single();
    commands.spawn((
        MovingObjectBundle {
            acceleration: Acceleration::new(acceleration),
            velocity: Velocity::new(velocity),
            model: SpriteBundle {
                transform: Transform {
                    translation: pit_transform.translation,
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgba(
                        1.0, 1.0, 1.0,
                        1.0, // alpha value, you can randomize this too if you want
                    ),
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..Default::default()
                },
                texture: textures.minion.clone(),
                ..Default::default()
            },
        },
        Inventory { stone: 0 },
        MinionInstructionSet { target: None },
        Minion,
    ));
}
