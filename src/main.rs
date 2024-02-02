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
        window_height: 1080 / 4,
        window_width: 1920 / 4,
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

    Player::add_to_scene(
        &mut scene,
        Vec2::new(screen_width() / 4., screen_height() / 4.),
        Vec2::new(10., 20.)
    );

    /*
     * NPC Collider
     */

    for i in 0..30 {
        if i == 5 {
            continue;
        }
        NPC::add_to_scene(
            &mut scene,
            Vec2::new(i as f32 * 40., screen_height() / 2.),
            Vec2::new(40., 40.)  
        );
    }

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