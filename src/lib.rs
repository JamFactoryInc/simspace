#![feature(trivial_bounds)]
#![feature(portable_simd)]
#![feature(iter_array_chunks)]
#![feature(array_chunks)]
#![feature(core_intrinsics)]

mod ecs;
mod entity;
pub mod util;

use godot::prelude::*;

struct SimSpaceExt;

#[gdextension]
unsafe impl ExtensionLibrary for SimSpaceExt { }
