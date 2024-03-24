use crate::assetloader::Textures;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::pit::{Pit, TribeGoal, TribeStoneQue};
use crate::stone::DestroyStone;
use crate::MinionUpdateSet;

use bevy::{math::*, prelude::*};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Inventory {
    stone: u16,
}

#[derive(Component)]
pub struct Minion;

#[derive(Component)]
pub struct MinionInstructionSet {
    //status: String,
    pub target: Option<Vec3>,
}
#[derive(Component)]

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(2.5, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_minion)
        .add_systems(
            Update,
            vague_minion_idiot_logic.in_set(MinionUpdateSet::PreDecisionMaking),
        );
    }
}

// Only deals with GET STONES Instructions
fn vague_minion_idiot_logic(
    mut minion_query: Query<(&mut Inventory, &mut MinionInstructionSet, &Transform), With<Minion>>,
    pit_query: Query<&Transform, With<Pit>>,
    goal: Res<TribeGoal>,
    mut stone_que: ResMut<TribeStoneQue>,
    mut stone_destroy_writer: EventWriter<DestroyStone>,
) {
    let mut pit_t = pit_query.single();
    for (mut m_inv, mut m_inst_set, m_t) in minion_query.iter_mut() {
        // Check inventory

        if (m_inv.stone > 0) {
            // this minion has stone, going back to pit
            m_inst_set.target = Some(pit_t.translation); //pit_t.translation.clone();
            continue;
        }
        // Check the latest stone from pit
        let next_stone_pos = stone_que.value.clone().into_iter().nth(0);

        if m_t.translation.x == next_stone_pos.unwrap().x
            && m_t.translation.y == next_stone_pos.unwrap().y
        {
            m_inv.stone + 1;

            print!("found");
        }
        m_inst_set.target = next_stone_pos;
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
