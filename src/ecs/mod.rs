use std::ops::{Add, Deref, Rem};

use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::{Schedule, World};
use godot::engine::{Control, ControlVirtual, InputEvent, NodeExt};
use godot::prelude::{Base, Color, Gd, godot_api, GodotClass, Node2DVirtual, NodePath, Rect2, Vector2};

use crate::ecs::physics::{CONTAINER, physics, Position, Velocity};
use crate::util::time;

mod physics;

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
    frame_count: usize,
}

#[derive(GodotClass)]
#[class(base=Control)]
struct EcsFramework {
    #[base]
    base: Base<Control>,
    world: World,
    timer: EcsTimer
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
    
    fn draw_entities(&mut self) {
        self.world.query::<&Position>().iter(&self.world).for_each(|position| {
            self.base.draw_circle(
                Vector2 {
                    x: position.x,
                    y: CONTAINER.1.y - position.y
                },
                10.0,
                Color::from_rgb(1.0, 0.0, 0.0)
            );
        })
    }
    
    fn draw_timings(&mut self) {
        self.base.draw_string(
            self.base.get_theme_font("default".into()).unwrap(),
             Vector2 {
                x: self.base.get_viewport_rect().size.x - 250.0,
                y: 30.0,
            },
            format!(
                "phys: {:?}us, render: {:?}us",
                self.timer.physics_micros,
                self.timer.render_micros
            ).into()
        )
    }
}

#[godot_api]
impl ControlVirtual for EcsFramework {
    fn init(base: Base<Control>) -> Self {
        
        // let mut app = App::new();
        // app.add_plugins(DefaultPlugins);
        // app.add_systems(SystemLabel::Physics, movement);
        
        let mut physics_schedule = Schedule::new();
        physics_schedule.add_systems(physics);
        
        let mut world = World::new();
        world.add_schedule(physics_schedule, Schedules::Physics);
        
        Self {
            base,
            world,
            timer: EcsTimer::default()
        }
    }
    
    fn ready(&mut self) {
        self.world.spawn(
            (
                Position(Vector2::new(100.0, 100.0)),
                Velocity(Vector2::new(10.0, 0.0))
            )
        );
        self.base.set_physics_process(false);
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
        self.timer.render_micros_rolling += time(|| {
            self.draw_bounding_box();
            self.draw_entities();
        }).0;
        
        self.draw_timings()
    }
    
    fn physics_process(&mut self, _: f64) {
        self.timer.frame_count = self.timer.frame_count
            .add(1)
            .rem(16);
        
        if self.timer.frame_count == 0 {
            self.timer.physics_micros = self.timer.physics_micros_rolling / 16;
            self.timer.physics_micros_rolling = 0;
            self.timer.render_micros = self.timer.render_micros_rolling / 16;
            self.timer.render_micros_rolling = 0;
        }
        
        self.timer.physics_micros_rolling += time(|| {
            self.world.run_schedule(Schedules::Physics);
            self.base.queue_redraw();
        }).0;
    }
}