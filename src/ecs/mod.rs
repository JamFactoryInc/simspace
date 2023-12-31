use std::ops::{Add, Deref, Rem};

use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::{Schedule, World};
use godot::builtin::PackedFloat32Array;
use godot::engine::{Control, ControlVirtual, InputEvent, MultiMesh, MultiMeshInstance2D, NodeExt, RenderingServer};
use godot::engine::utilities::randf_range;
use godot::prelude::{Base, Color, Gd, godot_api, GodotClass, NodePath, Rect2, Vector2};

use crate::ecs::physics::{CONTAINER, Position, Velocity};
use crate::util::time;

use self::physics::do_physics;

mod physics;

const INSTANCES: usize = 500000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(ScheduleLabel)]
enum Schedules {
    Physics,
}

#[derive(Default)]
struct EcsTimer {
    physics_micros: u64,
    physics_micros_rolling: u64,
    render_micros: u64,
    render_micros_rolling: u64,
    frame_micros: u64,
    frame_micros_rolling: u64,
    frame_count: usize,
    tick_count: usize,
}

#[derive(GodotClass)]
#[class(base=Control)]
struct EcsFramework {
    #[base]
    base: Base<Control>,
    world: World,
    timer: EcsTimer,
    multi_mesh: Option<Gd<MultiMesh>>,
    multi_mesh_buffer: PackedFloat32Array,
    render_server: Gd<RenderingServer>,
}

impl EcsFramework {
    pub fn get_instance<T: NodeExt>(node: &T) -> Gd<EcsFramework> {
        node.get_node_as::<EcsFramework>(NodePath::from("/root/Ecs"))
    }
    
    fn draw_bounding_box(&mut self) {
        self.base.draw_rect(
            Rect2::from_corners(
                CONTAINER.0,
                CONTAINER.1
            ),
            Color::from_rgb(0.0, 0.0, 0.0)
        );
    }
    
    fn update_mesh(&mut self) {
        self.multi_mesh_buffer.as_mut_slice().array_chunks_mut::<8>()
            .zip(self.world.query::<(&mut Position, &mut Velocity)>().iter_mut(&mut self.world))
            .for_each(|(transform, (mut pos, mut vel))| {

                do_physics(&mut pos, &mut vel);

                transform[3] = pos.x;
                transform[7] = pos.y;
            });
        
        self.render_server.multimesh_set_buffer(
            self.multi_mesh.as_ref().unwrap().get_rid(),
            self.multi_mesh_buffer.clone()
        );
    }
    
    fn draw_timings(&mut self) {
        self.base.draw_multiline_string(
            self.base.get_theme_font("default".into()).unwrap(),
            Vector2 {
                x: self.base.get_viewport_rect().size.x - 250.0,
                y: 30.0,
            },
            format!(
                "fps: {:?}\nphysics: {:?}us\nmesh update: {:?}us",
                1000000.0 / self.timer.frame_micros as f32,
                self.timer.physics_micros,
                self.timer.render_micros,
            ).into(),
        )
    }
}

#[godot_api]
impl ControlVirtual for EcsFramework {
    fn init(base: Base<Control>) -> Self {
        
        let mut physics_schedule = Schedule::new();
        
        let mut world = World::new();
        world.add_schedule(physics_schedule, Schedules::Physics);
        
        Self {
            base,
            world,
            timer: EcsTimer::default(),
            multi_mesh: None,
            multi_mesh_buffer: PackedFloat32Array::from(vec![0f32; INSTANCES * 8].as_slice()),
            render_server: RenderingServer::singleton()
        }
    }
    
    fn ready(&mut self) {
        
        self.world.spawn_batch(
            (0..INSTANCES).map(|_| {
                (
                    Position(Vector2::new(
                        randf_range(CONTAINER.0.x as f64, CONTAINER.1.x as f64) as f32,
                        randf_range(CONTAINER.0.y as f64, CONTAINER.1.y as f64) as f32,
                    )),
                    Velocity(Vector2::new(
                        randf_range(0.0, 2.0) as f32,
                        randf_range(0.0, 2.0) as f32
                    ))
                )
            })
        );
        self.base.set_physics_process(false);
        self.multi_mesh = self.base.get_child(0)
            .unwrap()
            .cast::<MultiMeshInstance2D>()
            .get_multimesh();
        
        if let Some(mesh) = self.multi_mesh.as_mut() {
            mesh.set_instance_count(INSTANCES as i32);
            mesh.set_visible_instance_count(INSTANCES as i32);
        }
        
        self.multi_mesh_buffer.as_mut_slice().chunks_exact_mut(8)
            .for_each(|transform| {
                transform[0] = 2.0;
                transform[5] = 2.0;
            });

        self.update_mesh();
    }
    
    fn unhandled_key_input(&mut self, event: Gd<InputEvent>) {
        if event.deref().is_action_pressed("Start Sim".into()) {
            self.base.set_physics_process(true);
        } else if event.deref().is_action_pressed("Stop Sim".into()) {
            self.base.set_physics_process(false);
        } else if event.deref().is_action_pressed("Toggle Sim".into()) {
            let is_processing = self.base.is_physics_processing();
            self.base.set_physics_process(!is_processing);
        }
    }
    
    fn draw(&mut self) {
        self.draw_bounding_box();
        self.draw_timings();
    }

    fn process(&mut self, delta: f64) {
        self.timer.frame_micros_rolling += (delta * 1000000.0) as u64;

        self.timer.frame_count = self.timer.frame_count
            .add(1)
            .rem(16);

        if self.timer.frame_count == 0 {
            self.timer.frame_micros = self.timer.frame_micros_rolling / 16;
            self.timer.frame_micros_rolling = 0;
        }
    }
    
    fn physics_process(&mut self, _: f64) {
        self.timer.tick_count = self.timer.tick_count
            .add(1)
            .rem(16);
        
        if self.timer.tick_count == 0 {
            self.timer.physics_micros = self.timer.physics_micros_rolling / 16;
            self.timer.physics_micros_rolling = 0;
            self.timer.render_micros = self.timer.render_micros_rolling / 16;
            self.timer.render_micros_rolling = 0;
        }
        
        self.timer.physics_micros_rolling += time(|| {
            //self.world.run_schedule(Schedules::Physics);
            self.base.queue_redraw();
        }).0;

        self.timer.render_micros_rolling += time(|| {
            self.update_mesh();
        }).0;
    }
}