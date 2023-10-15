#![feature(trivial_bounds)]

mod ecs;
mod entity;
pub mod util;

use godot::prelude::*;

struct SimSpaceExt;

#[gdextension]
unsafe impl ExtensionLibrary for SimSpaceExt { }
