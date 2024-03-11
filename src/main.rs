//! A simplified implementation of the classic game "Breakout".

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

mod assetloader;
mod minions;
mod mouse;
mod movement;
use assetloader::AssetLoaderPlugin;
use minions::MinionPlugin;
use mouse::MousePlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(MinionPlugin)
        .add_plugins(MousePlugin)
        .run();
}
