use std::ops::{Deref, DerefMut, Sub};
use bevy::prelude::{Bundle, Component, Deref, DerefMut, Query};
use godot::prelude::Vector2;

pub const GRAVITY: Vector2 = Vector2 { x: 0.0, y: 0.98 };
pub const CONTAINER: (Vector2, Vector2) = (
    Vector2::ZERO,
    Vector2 {
        x: 500.0,
        y: 500.0
    }
);

#[derive(Component, Deref, DerefMut)]
pub struct Position(pub Vector2);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vector2);

#[derive(Bundle)]
struct PhysicsBundle {
    position: Position,
    velocity: Velocity,
}

pub fn physics(mut query: Query<(&mut Position, &mut Velocity)>) {
    for (mut position, mut velocity) in &mut query {
        // move
        **position += **velocity;
        
        // adjust for
        if position.x < CONTAINER.0.x {
            position.x -= CONTAINER.0.x.sub(position.x).copysign(-velocity.x)
        }
        if position.x > CONTAINER.1.x {
            position.x -= CONTAINER.1.x.sub(position.x).copysign(-velocity.x)
        }
        if position.y < CONTAINER.0.y {
            position.y -= CONTAINER.0.y.sub(position.y).copysign(-velocity.y)
        }
        if position.y > CONTAINER.1.y {
            position.y -= CONTAINER.1.y.sub(position.y).copysign(-velocity.y)
        }
        
        // bounce
        velocity.x = velocity.x.copysign(velocity.x * (CONTAINER.1.x - position.x) * (position.x - CONTAINER.0.x));
        velocity.y = velocity.y.copysign(velocity.y * (CONTAINER.1.y - position.y) * (position.y - CONTAINER.0.y));
        
        // accelerate
        **velocity += GRAVITY;
    }
}