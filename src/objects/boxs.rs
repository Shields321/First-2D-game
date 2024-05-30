use minifb::Key;
use crate::game_engine::{
    game_object::{CollisionShape, GameObject, GameObjectCommon},
    types::XYPair,
};

pub struct Box {
    length: f64,
    width: f64,
    color: u32,

    common: GameObjectCommon,
}

impl Box{
    pub fn new(coords: XYPair, length: f64,width: f64, color_hex: &str) -> Self{
        let length = length;
        let width = width;
        let color = u32::from_str_radix(&color_hex[1..], 16).unwrap_or(0xFFFFFF);

        let common = GameObjectCommon{
            coords,
            ..GameObjectCommon::default()
        };

        Self {            
            length,
            width,
            color,

            common,
        }

    }
}
impl GameObject for Box{
    fn collision_shape(&self) -> CollisionShape {
        CollisionShape::Rectangle(self.width,self.length)
    }
    //fix the draw code
    fn draw(&self) -> Vec<Vec<u32>> {
        let mut raster = vec![vec![0; self.length as usize]; self.width as usize];
        let h = self.width;
        let k = self.length;

        for y in 0..self.width as usize {
            for x in 0..self.length as usize {
                let dx = (x as f64 - h).abs();
                let dy = (y as f64 - k).abs();
                if (dx*dy) <= self.width*self.length {
                    raster[y][x] = self.color;
                }
            }
        }

        raster
    }
    
    fn bounciness(&self) -> f64 {
        crate::game_engine::constants::DEFAULT_COLLISION_DAMPING_FACTOR
    }
    
    fn handle_input(&mut self, _keys: &[Key]) {}
    
    fn common(&mut self) -> &mut GameObjectCommon {
        &mut self.common
    }
    
    fn weight_factor(&self) -> f64 {
        0.0
    }
    
}