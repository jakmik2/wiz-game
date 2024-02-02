use macroquad::prelude::*;

use crate::Component;

pub struct NPC {
    pub id: String,
    pub pos: Vec2,
    pub size: Vec2
}

impl Component for NPC {
    // fn physics_process(&mut self, dt: f32, colliders: &rapier2d::prelude::ColliderSet, bodies: &mut rapier2d::prelude::RigidBodySet, queries: &rapier2d::prelude::QueryPipeline) -> () {
    //     miniquad::debug!("At pos {:?}", self.pos);
    // }

    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn assign_id(&mut self, id: &str) -> () {
        self.id = id.to_string();
    }
}