use godot::engine::{Sprite2D, Sprite2DVirtual};
use godot::prelude::{Base, godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Entity {
    #[base]
    base: Base<Sprite2D>,
}

#[godot_api]
impl Sprite2DVirtual for Entity {
    fn init(base: Base<Sprite2D>) -> Self {
        
        Self {
            base,
        }
    }
    
    // fn physics_process(&mut self, _: f64) {
    //     let ecs_f = self.base.get_node_as::<EcsFramework>(NodePath::from("/root/Ecs"));
    //     godot_print!("Test {ecs_f}");
    //     self.schedule.run(&mut self.world);
    // }
}