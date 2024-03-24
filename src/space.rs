use std::collections::HashMap;

use bevy::{prelude::*, transform::commands};

use crate::{minions::Minion, mouse::WorldLastClicked, pit::Pit};

#[derive(Resource)]
pub struct SpaceMap {
    pub value: HashMap<i32, Entity>,
}

pub fn map_vec3_hash(v: &Vec3) -> i32 {
    1920 * (v.x as i32) + (v.y as i32)
}

impl Default for SpaceMap {
    fn default() -> Self {
        SpaceMap {
            value: HashMap::new(),
        }
    }
}

#[derive(Event)]
pub struct AddSpatialEntity {
    pub entity: Entity,
    pub value: Vec3,
}

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpaceMap>()
            .add_event::<AddSpatialEntity>()
            .add_systems(Update, add_spatial_entity_sys);
    }
}

fn add_spatial_entity_sys(
    mut ev_add_entity: EventReader<AddSpatialEntity>,
    mut space_map: ResMut<SpaceMap>,
) {
    for ev in ev_add_entity.read() {
        println!("event received! {}", ev.value);

        space_map.value.insert(map_vec3_hash(&ev.value), ev.entity);
        //space_map.value.push(ev.entity);
    }
}
