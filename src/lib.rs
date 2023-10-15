#![feature(trivial_bounds)]

mod ecs;
mod entity;

use godot::prelude::*;

struct SimSpaceExt;

#[gdextension]
unsafe impl ExtensionLibrary for SimSpaceExt { }
