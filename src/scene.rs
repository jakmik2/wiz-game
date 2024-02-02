use std::collections::{HashMap, HashSet};
use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{Component};

pub struct Scene {
    // Map uuid for all entities
    components: HashMap<String, Box<dyn Component>>,
    physics_pipeline: PhysicsPipeline,
    colliders: ColliderSet,
    bodies: RigidBodySet,
    queries: QueryPipeline,
}

impl Scene {

    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            physics_pipeline: PhysicsPipeline::new(),
            colliders: ColliderSet::new(),
            bodies: RigidBodySet::new(),
            queries: QueryPipeline::new()
        }
    }

    pub fn push_body(&mut self, rbb: RigidBodyBuilder) -> RigidBodyHandle {
        self.bodies.insert(rbb)
    }

    pub fn push_collider(
            &mut self, 
            mut component: Box<dyn Component>, 
            handle: RigidBodyHandle,
            collider: Collider
        ) -> () {
        component.assign_id((self.components.len() + 1).to_string().as_str());

        self.colliders.insert_with_parent(collider, handle, &mut self.bodies);
        self.components.insert(component.get_id(), component);
    }

    pub fn call_ready(&mut self) -> () {
        
        for component in self.components.values_mut() {
            component.ready();
        }
    }

    pub fn call_physics(&mut self, dt: f32) -> () {
        for component in self.components.values_mut() {
            component.physics_process(dt, &self.colliders, &mut self.bodies, &self.queries);
        }

        // Bunch of facking vars
        let gravity = vector![0.0, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();
        
        // Run pipeline
        self.physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut self.queries),
            &physics_hooks,
            &event_handler,
          );
    }

    pub fn call_draw(&self) -> () {
        for component in self.components.values() {
            component.draw();
        }
    }
}