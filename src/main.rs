//! A simplified implementation of the classic game "Breakout".

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*
};

mod minions;
mod mouse;
use minions::MinionPlugin;
use mouse::MousePlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MinionPlugin)
        .add_plugins(MousePlugin)
        .run();
}
