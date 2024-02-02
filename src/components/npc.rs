use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{Component, Scene};

pub struct NPC {
    pub id: String,
    pub pos: Vec2,
    pub size: Vec2
}

impl NPC {
    pub fn add_to_scene(scene: &mut Scene, pos: Vec2, size: Vec2) {
        let npc = NPC {
            id: "".to_string(),
            pos,
            size
        };
    
        let npc_rb = RigidBodyBuilder::fixed().translation(vector![npc.pos.x, npc.pos.y]);
        let npc_handle = scene.push_body(npc_rb);
        let npc_collider = ColliderBuilder::cuboid(npc.size.x / 2., npc.size.y / 2.).build();
        
        let npc_box = Box::new(npc);
        
        scene.push_collider(npc_box, npc_handle, npc_collider);
    }
}

impl Component for NPC {
    // fn physics_process(&mut self, dt: f32, colliders: &rapier2d::prelude::ColliderSet, bodies: &mut rapier2d::prelude::RigidBodySet, queries: &rapier2d::prelude::QueryPipeline) -> () {
    //     miniquad::debug!("At pos {:?}", self.pos);
    // }
    fn ready(&mut self) -> () {
        miniquad::debug!("NPC has entered the scene");
    }

    // fn draw(&self) {
    //     // Default draws circle
    //     let pos = self.get_pos();
    //     let size = self.get_size();
    //     draw_rectangle(pos.x - 15., pos.y - 10., size.x, size.y, YELLOW)
    
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