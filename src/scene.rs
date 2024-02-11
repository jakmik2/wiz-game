use macroquad::prelude::*;
use rapier2d::{crossbeam, prelude::*};
use macroquad::prelude::*;
use rapier2d::{crossbeam, prelude::*};
use std::collections::{HashMap, HashSet};

use crate::Component;

pub struct Scene {
    component_scale: f32,
    component_scale: f32,
    // Map uuid for all entities
    components: HashMap<ColliderHandle, Box<dyn Component>>,
    components: HashMap<ColliderHandle, Box<dyn Component>>,
    physics_pipeline: PhysicsPipeline,
    colliders: ColliderSet,
    pub bodies: RigidBodySet,
    pub bodies: RigidBodySet,
    queries: QueryPipeline,
    narrow_phase: NarrowPhase,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    narrow_phase: NarrowPhase,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            component_scale: 60.,
            component_scale: 60.,
            components: HashMap::new(),
            physics_pipeline: PhysicsPipeline::new(),
            colliders: ColliderSet::new(),
            bodies: RigidBodySet::new(),
            queries: QueryPipeline::new(),
            narrow_phase: NarrowPhase::new(),

            // Bunch of facking vars
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            queries: QueryPipeline::new(),
            narrow_phase: NarrowPhase::new(),

            // Bunch of facking vars
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }

    pub fn push_body(&mut self, rbb: RigidBodyBuilder) -> RigidBodyHandle {
        self.bodies.insert(rbb)
    }

    pub fn push_collider(&mut self, component: Box<dyn Component>, collider: Collider) -> () {
        let collider_handle = self.colliders.insert(collider);
        self.components.insert(collider_handle, component);

        let component = self.components.get_mut(&collider_handle).unwrap();
        component.assign_collider_handle(Some(collider_handle));
    }

    pub fn push_collider_with_rb(
        &mut self,
        component: Box<dyn Component>,
        handle: RigidBodyHandle,
        collider: Collider,
    ) -> () {
        let collider_handle = self
            .colliders
            .insert_with_parent(collider, handle, &mut self.bodies);
        self.components.insert(collider_handle, component);

        let component = self.components.get_mut(&collider_handle).unwrap();
        component.assign_collider_handle(Some(collider_handle));
    }

    pub fn call_ready(&mut self) -> () {
        for component in self.components.values_mut() {
            component.ready();
        }
    }

    pub fn call_physics(&mut self, dt: f32) -> () {
        for component in self.components.values_mut() {
            component.physics_process(
                dt,
                &self.colliders,
                &mut self.bodies,
                &self.queries,
                &self.narrow_phase,
            );
        }
        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        let gravity = vector![0.0, 0.0];


        let physics_hooks = ();


        // Run pipeline
        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.queries),
            &physics_hooks,
            &event_handler,
        );

        // while let Ok(collision_event) = collision_recv.try_recv() {
        //     miniquad::debug!("Collision Event: {:?}", collision_event);
        // }

        // while let Ok(contact_force_event) = contact_force_recv.try_recv() {
        //     miniquad::debug!("Contact event: {:?}", contact_force_event);
        // }
        for component in self.components.values_mut() {
            for (collider1, collider2, intersecting) in self
                .narrow_phase
                .intersection_pairs_with(component.get_collider_handle())
            {
                if intersecting {
                    miniquad::debug!(
                        "The colliders {:?} and {:?} are intersecting!",
                        collider1,
                        collider2
                    );
                }
            }
        }
    }

    pub fn call_draw(&self) -> () {
        for component in self.components.values() {
            component.draw(None);
        }
    }
}

