pub mod components;
pub mod systems;
pub mod states;
mod scene;

use std::collections::VecDeque;

use macroquad::prelude::*;

use components::prelude::*;
use rapier2d::{control::KinematicCharacterController, geometry::Collider, na::Point, parry::shape::{Capsule, Cuboid}, prelude::*};
use systems::prelude::*;
use scene::*;

const MAP_SIZE: i16 = 64;

struct GameState {
    game_over: bool,
    load_scene_tree: bool,
    level: i8,
}

struct Deck {
    cards: Vec<Box<dyn Card>>
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Wiz Game".to_owned(),
        window_height: 1080,
        window_width: 1920,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState {
        game_over: true,
        load_scene_tree: false,
        level: 0,
    };

    /*
     * World
     */
    let mut scene = Scene::new();
    
    /*
     * Player
     */

    // Need One source of truth for position and size 
    let player_rigid_body: RigidBodyBuilder = RigidBodyBuilder::kinematic_position_based().translation(vector![screen_height() / 4., screen_width() / 4.]);
    let player_handle: RigidBodyHandle = scene.push_body(player_rigid_body);
    let collider: Collider = ColliderBuilder::cuboid(2.5, 2.5).build();
 
    let player = Player {
        id: "".to_string(),
        pos: Vec2::new(screen_height() / 4., screen_width() / 4.),
        size: Vec2::new(5., 5.),
        velocity: 5.,
        character_handle: player_handle,
        character_controller: KinematicCharacterController::default(),
    };

    let player_box = Box::new(player);
    
    scene.push_collider(player_box, player_handle, collider);

    /*
     * NPC Collider
     */
    let npc = NPC {
        id: "".to_string(),
        pos: Vec2::new(screen_width() / 2. - 340., screen_height() / 2. + 40.),
        size: Vec2::new(5., 5.),
    };

    let npc_rb = RigidBodyBuilder::fixed().translation(vector![npc.pos.x, npc.pos.y]);
    let npc_handle = scene.push_body(npc_rb);
    let npc_collider = ColliderBuilder::cuboid(npc.size.x / 2., npc.size.y / 2.).build();
    
    let npc_box = Box::new(npc);
    
    scene.push_collider(npc_box, npc_handle, npc_collider);

    let mut fps_times: VecDeque<i32> = VecDeque::new();

    let mut physics_manager = PhysicsManager::new();

    loop {
        if game_state.game_over {
            // REGION : START SCREEN
            clear_background(WHITE);

            let font_size = 40.;

            let text = "Press [Enter] to start Game";
            
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                game_state.game_over = false;
            }
        } else {
            // REGION : LOAD SCENE TREE
            if !game_state.load_scene_tree {
                scene.call_ready();
                game_state.load_scene_tree = true;
            }

            // REGION : PHYSICS PROCESSES
            if physics_manager.run() {
                scene.call_physics(physics_manager.last_frame);

                // CHECK COLLISIONS
                
            }

            // REGION : FRAME PROCESSES

            // SUB REGION : DRAW
            scene.call_draw();
        }

        // REGION END EVERY SCENE

        let new_fps = get_fps();

        if new_fps > 0 && new_fps < 1000 {
            fps_times.push_front(new_fps);
        }

        if fps_times.len() > 100 {
            let _popped = fps_times.pop_back();
        }

        let fps = format!("AVG of last 100 FPS: {:.0}", fps_times.clone().into_iter().reduce(|a, b| (a.wrapping_add(b))).unwrap_or_default() / fps_times.len() as i32);

        let fps_text_size = measure_text(fps.as_str(), None, 32. as _, 1.0);

        draw_text(fps.as_str(), 10., fps_text_size.height + 10., 32., DARKGRAY);
        
        next_frame().await;
    }
}