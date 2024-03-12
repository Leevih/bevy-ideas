//! A simplified implementation of the classic game "Breakout".

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

mod assetloader;
mod minions;
mod mouse;
mod movement;
mod pit;
mod stone;

use assetloader::AssetLoaderPlugin;
use minions::MinionPlugin;
use mouse::MousePlugin;
use movement::MovementPlugin;
use pit::PitPlugin;
use stone::StonePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(MovementPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(PitPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MousePlugin)
        .add_plugins(MinionPlugin)
        .add_plugins(StonePlugin)
        .run();
}
