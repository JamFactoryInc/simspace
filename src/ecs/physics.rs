use std::ops::{Sub};
use bevy::prelude::{Component, Deref, DerefMut, Query, Mut};
use godot::log::godot_print;
use godot::prelude::Vector2;

pub const GRAVITY: Vector2 = Vector2 { x: 0.0, y: 0.1 };
pub const CONTAINER: (Vector2, Vector2) = (
    Vector2 {
        x: 0.0,
        y: 0.0
    },
    Vector2 {
        x: 1000.0,
        y: 1000.0
    }
);

#[derive(Component, Deref, DerefMut, Copy, Clone)]
pub struct Position(pub Vector2);
unsafe impl Sync for Position { }

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vector2);
unsafe impl Sync for Velocity { }

#[inline(always)]
pub fn do_physics(position: &mut Position, mut velocity: &mut Velocity) {
    // move
    **position += **velocity;
    
    // bounce
    velocity.x = velocity.x.copysign(velocity.x * (CONTAINER.1.x - position.x) * (position.x - CONTAINER.0.x));
    velocity.y = velocity.y.copysign(velocity.y * (CONTAINER.1.y - position.y) * (position.y - CONTAINER.0.y));
    
    // adjust for when the velocity step moves past a bound
    if position.x < CONTAINER.0.x {
        position.x += 2.0 * CONTAINER.0.x.sub(position.x).copysign(velocity.x)
    }
    if position.x > CONTAINER.1.x {
        position.x += 2.0 * CONTAINER.1.x.sub(position.x).copysign(velocity.x)
    }
    if position.y < CONTAINER.0.y {
        position.y += 2.0 * CONTAINER.0.y.sub(position.y).copysign(velocity.y)
    }
    if position.y > CONTAINER.1.y {
        position.y += 2.0 * CONTAINER.1.y.sub(position.y).copysign(velocity.y)
    }
    
    // accelerate
    **velocity += GRAVITY;
}