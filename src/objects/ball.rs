use minifb::Key;
use lazy_static::lazy_static;
use std::sync::Mutex;


use crate::game_engine::{
    game_object::{CollisionShape, GameObject, GameObjectCommon},
    types::XYPair,
};

const KB_X_BOOST: f64 = 0.2;
const KB_Y_BOOST: f64 = 20.0;

lazy_static! {
    static ref GLOBAL_VAR: Mutex<String> = Mutex::new("".to_string());
}

pub struct Ball {
    radius: f64,
    diameter: f64,
    color: u32,

    common: GameObjectCommon,
}

impl Ball {
    pub fn new(coords: XYPair, radius: f64, color_hex: &str) -> Self {
        let diameter = radius * 2.0;
        let color = u32::from_str_radix(&color_hex[1..], 16).unwrap_or(0xFFFFFF);

        let common = GameObjectCommon {
            coords,
            ..GameObjectCommon::default()
        };

        Self {
            color,
            radius,
            diameter,

            common,
        }
    }
}

impl GameObject for Ball {
    fn weight_factor(&self) -> f64 {
        0.8
    }

    fn bounciness(&self) -> f64 {
        0.2
    }

    fn collision_shape(&self) -> CollisionShape {
        CollisionShape::Circle(self.radius)
    }

    fn common(&mut self) -> &mut GameObjectCommon {
        &mut self.common
    }

    fn draw(&self) -> Vec<Vec<u32>> {
        let mut raster = vec![vec![0; self.diameter as usize]; self.diameter as usize];
        let h = self.radius;
        let k = self.radius;

        for y in 0..self.diameter as usize {
            for x in 0..self.diameter as usize {
                let dx = (x as f64 - h).abs();
                let dy = (y as f64 - k).abs();
                if (dx * dx + dy * dy).sqrt() <= self.radius {
                    raster[y][x] = self.color;
                }
            }
        }

        raster
    }

    fn handle_input(&mut self, keys: &[Key]) {
        if keys.contains(&Key::A) || *GLOBAL_VAR.lock().unwrap() == "a"{
            self.common.velocities.x -= KB_X_BOOST;
        }

        if keys.contains(&Key::D) || *GLOBAL_VAR.lock().unwrap() == "d" {
            self.common.velocities.x += KB_X_BOOST;
        }            
        // jump if we are on the ground AND have 0 or lesser y velocity
        if keys.contains(&Key::W) || keys.contains(&Key::Space) || *GLOBAL_VAR.lock().unwrap() == "w"{
            if let Some(info) = &self.common.object_info {
                if self.common.velocities.y < 0.0
                    && self.common.coords.y + self.diameter == info.window_size.height as f64
                {
                    self.common.velocities.y -= KB_Y_BOOST;
                }
            }
        }
        if *GLOBAL_VAR.lock().unwrap() == "wa" || *GLOBAL_VAR.lock().unwrap() == "aw" {
            self.common.velocities.x -= KB_X_BOOST;
            if let Some(info) = &self.common.object_info {
                if self.common.velocities.y < 0.0
                    && self.common.coords.y + self.diameter == info.window_size.height as f64
                {
                    self.common.velocities.y -= KB_Y_BOOST;
                }
            }
        }
        if *GLOBAL_VAR.lock().unwrap() == "wd" || *GLOBAL_VAR.lock().unwrap() == "dw" {
            self.common.velocities.x += KB_X_BOOST;
            if let Some(info) = &self.common.object_info {
                if self.common.velocities.y < 0.0
                    && self.common.coords.y + self.diameter == info.window_size.height as f64
                {
                    self.common.velocities.y -= KB_Y_BOOST;
                }
            }
        }
    }
}
pub fn player_move(msg: &str) {
    // Lock the mutex to gain access to the global variable        
    *GLOBAL_VAR.lock().unwrap() = msg.to_string();  
      
}
