use macroquad::prelude::*;
use rapier2d::{control::{CharacterCollision, KinematicCharacterController}, parry::query, prelude::*};

use crate::Component;

pub struct Player {
    pub id: String,
    pub pos: Vec2,
    pub size: Vec2,
    pub velocity: f32,
    pub character_controller: KinematicCharacterController,
    pub character_handle: RigidBodyHandle
}

impl Component for Player {
    fn ready(&mut self) -> () {
        miniquad::debug!("Player has made it!");
        
    }

    fn physics_process(&mut self, dt: f32, colliders: &ColliderSet, bodies: &mut RigidBodySet, queries: &QueryPipeline) -> () {
        // Attempt Move
        let desired_translation = self.movement();

        if desired_translation == Vec2::ZERO {
            return;
        }

        let character_body = &bodies[self.character_handle];
        let character_collider = &colliders[character_body.colliders()[0]];
        let character_mass = character_body.mass();

        let mut collisions = vec![];

        // Correct Movement
        let mvt = self.character_controller.move_shape(
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
            miniquad::debug!("Collided: {:?}", collision);
            self.character_controller.solve_character_collision_impulses(
                dt,
                bodies, 
                colliders, 
                queries, 
                character_collider.shape(), 
                character_mass, 
                collision, 
                QueryFilter::new().exclude_rigid_body(self.character_handle)
            )
        }

        // TODO : track position in one source of truth
        let character_body = &mut bodies[self.character_handle];
        miniquad::debug!("{:?}", &mvt.translation);

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

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn assign_id(&mut self, id: &str) -> () {
        self.id = id.to_string();
    }
}

impl Player {
    pub fn movement(&mut self) -> Vec2 {
        let mut translation = Vec2::ZERO;

        match get_last_key_pressed() {
            Some(key) => miniquad::debug!("{:?}", key),
            None => ()
        };

        if is_mouse_button_down(MouseButton::Left) {
            miniquad::debug!("LMB pressed");
            miniquad::debug!("PLAYER WITH ID : {:?}", self.get_id());
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