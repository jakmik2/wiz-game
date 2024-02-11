use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{Component, Scene};

pub struct NPC {
    pub pos: Vec2,
    pub size: Vec2,
    collider_handle: Option<ColliderHandle>,
}

impl NPC {
    pub fn add_to_scene(scene: &mut Scene, pos: Vec2, size: Vec2, rb: bool) {
        let mut npc_collider = ColliderBuilder::cuboid(size.x / 2., size.y / 2.).build();

        let npc = NPC {
            pos,
            size,
            collider_handle: None,
        };

        let npc_box = Box::new(npc);

        if rb {
            let npc_rb = RigidBodyBuilder::fixed().translation(vector![pos.x, pos.y]);
            let npc_handle = scene.push_body(npc_rb);

            scene.push_collider_with_rb(npc_box, npc_handle, npc_collider);
        } else {
            npc_collider.set_sensor(true);
            scene.push_collider(npc_box, npc_collider);
        }
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

    fn get_collider_handle(&self) -> ColliderHandle {
        self.collider_handle.unwrap()
    fn get_collider_handle(&self) -> ColliderHandle {
        self.collider_handle.unwrap()
    }

    fn assign_collider_handle(&mut self, collider_handle: Option<ColliderHandle>) -> () {
        self.collider_handle = collider_handle;
    }
}

