mod physics;

use std::ops::Deref;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::{Schedule, World};
 use godot::engine::{InputEvent, NodeExt};
use godot::prelude::{Base, Color, Gd, godot_api, GodotClass, Node2D, Node2DVirtual, NodePath, Rect2, Vector2};
use crate::ecs::physics::{CONTAINER, physics, Position, Velocity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(ScheduleLabel)]
enum Schedules {
    Physics,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct EcsFramework {
    #[base]
    base: Base<Node2D>,
    world: World,
}

impl EcsFramework {
    pub fn get_instance<T: NodeExt>(node: &T) -> Gd<EcsFramework> {
        node.get_node_as::<EcsFramework>(NodePath::from("/root/Ecs"))
    }
}

#[godot_api]
impl Node2DVirtual for EcsFramework {
    fn init(base: Base<Node2D>) -> Self {
        
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
        }
    }
    
    fn ready(&mut self) {
        self.world.spawn(
            (
                Position(Vector2::new(100.0, 100.0)),
                Velocity(Vector2::new(0.0, 0.0))
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
        // draw bounding box
        self.base.draw_rect(
            Rect2::from_corners(
                CONTAINER.0,
                CONTAINER.1
            ),
            Color::from_rgb(0.0, 0.0, 0.0)
        );
        
        // draw entities
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
    
    fn physics_process(&mut self, _: f64) {
        self.world.run_schedule(Schedules::Physics);
        self.base.queue_redraw();
    }
}