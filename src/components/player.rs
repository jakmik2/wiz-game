use macroquad::prelude::*;
use rapier2d::{
    control::{CharacterCollision, KinematicCharacterController},
    parry::query,
    prelude::*,
};
use rapier2d::{
    control::{CharacterCollision, KinematicCharacterController},
    parry::query,
    prelude::*,
};

use crate::{Component, Scene};

pub struct Player {
    pub pos: Vec2,
    pub size: Vec2,
    pub velocity: f32,
    pub character_handle: RigidBodyHandle,
    pub collider_handle: Option<ColliderHandle>,
    pub character_handle: RigidBodyHandle,
    pub collider_handle: Option<ColliderHandle>,
}

impl Player {
    pub fn add_to_scene(scene: &mut Scene, pos: Vec2, size: Vec2) {
        let player_rigid_body: RigidBodyBuilder =
            RigidBodyBuilder::kinematic_position_based().translation(vector![pos.x, pos.y]);
        let player_handle: RigidBodyHandle = scene.push_body(player_rigid_body);
        let collider: Collider = ColliderBuilder::cuboid(size.x / 2., size.y / 2.).build();

        let player = Player {
            pos,
            size,
            velocity: 5.,
            character_handle: player_handle,
            collider_handle: None,
            collider_handle: None,
        };

        let player_box = Box::new(player);

        scene.push_collider_with_rb(player_box, player_handle, collider);

        scene.push_collider_with_rb(player_box, player_handle, collider);
    }
}

impl Component for Player {
    fn ready(&mut self) -> () {
        miniquad::debug!("Player has entered scene it!");
    }

    fn physics_process(
        &mut self,
        dt: f32,
        colliders: &ColliderSet,
        bodies: &mut RigidBodySet,
        queries: &QueryPipeline,
    ) -> () {
        // Attempt Move
        let desired_translation = self.movement();

        if desired_translation == Vec2::ZERO {
            return;
        }

        let character_body = &bodies[self.character_handle];
        let character_collider = &colliders[character_body.colliders()[0]];
        let character_mass = character_body.mass();

        let mut collisions = vec![];

        let character_controller = KinematicCharacterController::default();

        // Correct Movement
        let mvt = character_controller.move_shape(
            dt,
            bodies,
            colliders,
            queries,
            character_collider.shape(),
            character_collider.position(),
            dt,
            bodies,
            colliders,
            queries,
            character_collider.shape(),
            character_collider.position(),
            vector![desired_translation.x, desired_translation.y],
            QueryFilter::new().exclude_rigid_body(self.character_handle),
            |c| collisions.push(c),
        );

        // Resolve collisions
        for collision in &collisions {
            // miniquad::debug!("Collided: {:?}", collision);
            character_controller.solve_character_collision_impulses(
                dt,
                bodies,
                colliders,
                queries,
                character_collider.shape(),
                character_mass,
                collision,
                QueryFilter::new().exclude_rigid_body(self.character_handle),
                bodies,
                colliders,
                queries,
                character_collider.shape(),
                character_mass,
                collision,
                QueryFilter::new().exclude_rigid_body(self.character_handle),
            )
        }

        // TODO : track position in one source of truth
        let character_body = &mut bodies[self.character_handle];
        // miniquad::debug!("{:?}", &mvt.translation);

        let pos = character_body.position();
        let modified_pos = pos.translation.vector + mvt.translation;
        character_body.set_translation(modified_pos, true);

        self.pos = Vec2::new(modified_pos.x, modified_pos.y);
    }

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

impl Player {
    pub fn movement(&mut self) -> Vec2 {
        let mut translation = Vec2::ZERO;

        match get_last_key_pressed() {
            Some(key) => miniquad::debug!("{:?}", key),
            None => (),
            None => (),
        };

        if is_mouse_button_down(MouseButton::Left) {
            miniquad::debug!("LMB pressed");
        }

        // Update player position
        if is_key_down(KeyCode::W) {
            translation.y -= self.velocity;
        }


        if is_key_down(KeyCode::S) {
            translation.y += self.velocity;
        }

        if is_key_down(KeyCode::A) {
            translation.x -= self.velocity;
        }

        if is_key_down(KeyCode::D) {
            translation.x += self.velocity;
        }

        translation
    }
}

