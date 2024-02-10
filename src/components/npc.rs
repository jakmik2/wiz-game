use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{Component, Scene};

pub struct NPC {
    pub collider_handle: Option<ColliderHandle>,
    pub pos: Vec2,
    pub size: Vec2,
}

impl NPC {
    pub fn add_to_scene(scene: &mut Scene, pos: Vec2, size: Vec2) -> () {
        let npc = NPC {
            pos,
            size,
            collider_handle: None,
        };
        let npc_rbb: RigidBodyBuilder =
            RigidBodyBuilder::kinematic_position_based().translation(vector![pos.x, pos.y]);
        let npc_rbh: RigidBodyHandle = scene.push_body(npc_rbb);
        let npc_collider = ColliderBuilder::cuboid(npc.size.x / 2., npc.size.y / 2.).build();

        let npc_box = Box::new(npc);

        scene.push_collider_with_rb(npc_box, npc_rbh, npc_collider);
    }
}

impl Component for NPC {
    // fn physics_process(&mut self, dt: f32, colliders: &rapier2d::prelude::ColliderSet, bodies: &mut rapier2d::prelude::RigidBodySet, queries: &rapier2d::prelude::QueryPipeline) -> () {
    //     miniquad::debug!("At pos {:?}", self.pos);
    // }
    fn ready(&mut self) -> () {
        miniquad::debug!("NPC has entered the scene");
    }

    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_collider_handle(&self) -> ColliderHandle {
        self.collider_handle.unwrap()
    }

    fn assign_collider_handle(&mut self, collider_handle: ColliderHandle) -> () {
        self.collider_handle = Some(collider_handle);
    }
}
