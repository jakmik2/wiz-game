
use macroquad::prelude::*;

pub struct PhysicsManager {
    physics_tick: f32,
    frequency_threshold: f32,
    pub last_frame: f32
}

impl PhysicsManager {
    pub fn new() -> Self {
        Self {
            physics_tick: 0.,
            frequency_threshold: 0.01,
            last_frame: 0.,
        }
    }

    pub fn tick(&mut self) {
        self.physics_tick += get_frame_time();
    }

    pub fn run(&mut self) -> bool {
        self.tick();

        if self.physics_tick >= self.frequency_threshold {
            self.last_frame = self.physics_tick;
            self.physics_tick = 0.;
            return true;
        }
        return false;
    }
}