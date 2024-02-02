
use macroquad::prelude::*;
use rapier2d::{control::CharacterCollision, prelude::*};

pub trait Component {
    // Initialize Component Behavior
    fn ready(&mut self) -> () {}
    // Every Frame physics Process
    fn physics_process(&mut self, dt: f32, colliders: &ColliderSet, bodies: &mut RigidBodySet, queries: &QueryPipeline) -> () {}
    // Every Frame Draw
    fn draw(&self) {
        // Default draws circle
        let pos = self.get_pos();
        let size = self.get_size();
        draw_rectangle_ex(pos.x - size.x / 2., pos.y - size.y / 2., size.x, size.y, DrawRectangleParams {
            color: YELLOW,
            ..Default::default()
        });
    }
    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_id(&self) -> String;
    fn assign_id(&mut self, id: &str) -> ();
}