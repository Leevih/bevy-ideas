//! A simplified implementation of the classic game "Breakout".

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    window::WindowResolution,
};

mod assetloader;
mod minions;
mod mouse;
mod movement;
mod pit;
mod space;
mod stone;

use assetloader::AssetLoaderPlugin;
use minions::MinionPlugin;
use mouse::MousePlugin;
use movement::MovementPlugin;
use pit::PitPlugin;
use space::SpacePlugin;
use stone::StonePlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MinionUpdateSet {
    PreDecisionMaking,
    PostDecisionMaking,
}

fn main() {
    App::new()
        .configure_sets(
            Update,
            (
                MinionUpdateSet::PreDecisionMaking,
                MinionUpdateSet::PostDecisionMaking,
            )
                .chain(),
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(SpacePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(PitPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MousePlugin)
        .add_plugins(MinionPlugin)
        .add_plugins(StonePlugin)
        .run();
}
