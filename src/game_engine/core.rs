use minifb::{Key, ScaleMode, Window, WindowOptions};

use super::{
    constants::*,
    game_object::{CollisionShape, GameObject},
    types::{ObjectInfo, WindowSize, XYPair},
};

pub struct Engine {
    window: Option<Window>,
    buffer: Vec<u32>,
    window_size: WindowSize,
    objects: Vec<Box<dyn GameObject>>,
}
impl CollisionShape {
    fn height(&self) -> f64 {
        match self {
            CollisionShape::Circle(radius) => 2.0 * radius,
            CollisionShape::Rectangle(width, _length) => *width,
        }
    }
    fn check_collision(&self, other: &CollisionShape, coords_self: &XYPair, coords_other: &XYPair) -> bool {
        match (self, other) {
            (CollisionShape::Circle(radius1), CollisionShape::Circle(radius2)) => {
                Self::circle_circle_collision(*radius1, coords_self, *radius2, coords_other)
            }
            (CollisionShape::Circle(radius), CollisionShape::Rectangle(width, height)) |
            (CollisionShape::Rectangle(width, height), CollisionShape::Circle(radius)) => {
                Self::circle_rectangle_collision(*radius, coords_self, *width, *height, coords_other)
            }
            (CollisionShape::Rectangle(width1, height1), CollisionShape::Rectangle(width2, height2)) => {
                Self::rectangle_rectangle_collision(*width1, *height1, coords_self, *width2, *height2, coords_other)
            }
        }
    }

    fn circle_circle_collision(radius1: f64, coords1: &XYPair, radius2: f64, coords2: &XYPair) -> bool {
        let dx = coords1.x - coords2.x;
        let dy = coords1.y - coords2.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < radius1 + radius2
    }

    fn circle_rectangle_collision(circle_radius: f64, circle_coords: &XYPair, rect_width: f64, rect_height: f64, rect_coords: &XYPair) -> bool {
        let circle_distance_x = (circle_coords.x - rect_coords.x - rect_width / 2.0).abs();
        let circle_distance_y = (circle_coords.y - rect_coords.y - rect_height / 2.0).abs();

        if circle_distance_x > (rect_width / 2.0 + circle_radius) { return false; }
        if circle_distance_y > (rect_height / 2.0 + circle_radius) { return false; }

        if circle_distance_x <= (rect_width / 2.0) { return true; }
        if circle_distance_y <= (rect_height / 2.0) { return true; }

        let corner_distance_sq = (circle_distance_x - rect_width / 2.0).powi(2) +
                                 (circle_distance_y - rect_height / 2.0).powi(2);

        corner_distance_sq <= circle_radius.powi(2)
    }

    fn rectangle_rectangle_collision(width1: f64, height1: f64, coords1: &XYPair, width2: f64, height2: f64, coords2: &XYPair) -> bool {
        let x_overlap = coords1.x < coords2.x + width2 && coords1.x + width1 > coords2.x;
        let y_overlap = coords1.y < coords2.y + height2 && coords1.y + height1 > coords2.y;
        x_overlap && y_overlap
    }
}
// public functions
impl Engine {
    pub fn new(window_size: &WindowSize) -> Result<Self, anyhow::Error> {
        Ok(Self {
            buffer: vec![0; window_size.width * window_size.height],
            window: None,
            window_size: window_size.clone(),
            objects: Vec::new(),
        })
    }

    pub fn add_game_object(&mut self, game_object: impl GameObject + 'static) {
        self.objects.push(Box::new(game_object))
    }
}

// internal functions
impl Engine {            
    fn calc_velocities(object: &mut Box<dyn GameObject>) {
        let mut velocities = object.common().velocities.clone();

        // apply gravity
        let gravity = GRAVITY * object.weight_factor() * DT;
        velocities.y += gravity;

        // apply air drag
        velocities.x *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);
        velocities.y *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);

        object.common().velocities = velocities;
    }

    fn apply_velocities(object: &mut Box<dyn GameObject>) {
        let common = object.common();
        let coords = common.coords.clone();
        let velocities = common.velocities.clone();

        object.common().coords = XYPair {
            x: coords.x + velocities.x,
            y: coords.y + velocities.y,
        };
    } 
    fn collision_between(&mut self) {
        let objects_len = self.objects.len();
        for i in 0..objects_len {
            for j in (i + 1)..objects_len {
                let coords1 = self.objects[i].common().coords.clone();
                let coords2 = self.objects[j].common().coords.clone();
    
                let shape1 = self.objects[i].collision_shape();
                let shape2 = self.objects[j].collision_shape();
    
                if shape1.check_collision(&shape2, &coords1, &coords2) {
                    let mut vel1 = self.objects[i].common().velocities.clone();
                    let mut vel2 = self.objects[j].common().velocities.clone();
                    
                    let object1_above_object2 = coords1.y + shape1.height() <= coords2.y;
                    let object2_above_object1 = coords2.y + shape2.height() <= coords1.y;
                    match (shape1, shape2) {
                        (CollisionShape::Circle(radius), CollisionShape::Rectangle(width, height)) => {
                            let circle_bottom = coords1.y + radius;
                            let circle_top = coords1.y - radius;
                            let circle_left = coords1.x - radius;
                            let circle_right = coords1.x + radius;
                            
                            let rect_top = coords2.y;
                            let rect_bottom = coords2.y + width;
                            let rect_left = coords2.x;
                            let rect_right = coords2.x + height;
    
                            // Check if the circle is on top of the rectangle
                            if circle_bottom >= rect_top && circle_top < rect_top && coords1.x >= rect_left && coords1.x <= rect_right {
                                // Adjust the circle's position to rest on top of the rectangle
                                self.objects[i].common().coords.y = rect_top - radius;
                                // Stop the circle's vertical velocity
                                vel1.y = 0.0;
                                vel1.x *= GROUND_DRAG_FACTOR;
                                // Allow horizontal movement by responding to user input
                                if self.window.as_ref().unwrap().is_key_down(Key::A) {
                                    vel1.x = -CIRCLE_MOVE_SPEED;
                                }
                                if self.window.as_ref().unwrap().is_key_down(Key::D) {
                                    vel1.x = CIRCLE_MOVE_SPEED;
                                }
                                if self.window.as_ref().unwrap().is_key_down(Key::Space) || self.window.as_ref().unwrap().is_key_down(Key::W) {
                                    vel1.y = -JUMP_SPEED;
                                }
                            } else if circle_top < rect_bottom && circle_bottom > rect_bottom && coords1.x >= rect_left && coords1.x <= rect_right {
                                // Circle is hitting the bottom of the rectangle
                                self.objects[i].common().coords.y = rect_bottom + radius;
                                vel1.y = -vel1.y;
                            } else if circle_right >= rect_left && circle_left < rect_left && coords1.y >= rect_top && coords1.y <= rect_bottom {
                                // Circle is hitting the left side of the rectangle
                                self.objects[i].common().coords.x = rect_left - radius;
                                vel1.x = -vel1.x;
                            } else if circle_left <= rect_right && circle_right > rect_right && coords1.y >= rect_top && coords1.y <= rect_bottom {
                                // Circle is hitting the right side of the rectangle
                                self.objects[i].common().coords.x = rect_right + radius;
                                vel1.x = -vel1.x;
                            }
    
                            // Update velocities in the objects after the collision check
                            self.objects[i].common().velocities = vel1.clone();
                            self.objects[j].common().velocities = vel2.clone();
                        },
                        _ => {                            
    
                            if !object1_above_object2 && !object2_above_object1 {
                                vel1.x = -vel1.x;
                                vel1.y = -vel1.y;
                                vel2.x = -vel2.x;
                                vel2.y = -vel2.y;
    
                                // Update velocities in the objects after the collision check
                                self.objects[i].common().velocities = vel1.clone();
                                self.objects[j].common().velocities = vel2.clone();
                            }
                        }
                    }
                }
            }
        }
    }
    

    fn collision_checks(window_size: &WindowSize, object: &mut Box<dyn GameObject>) {
        match object.collision_shape() {
            CollisionShape::Circle(radius) => {
                let mut coords = object.common().coords.clone();
                let mut velocities = object.common().velocities.clone();
                let diameter = 2.0 * radius;

                let on_ground = if coords.y + diameter >= window_size.height as f64 {
                    true
                } else {
                    false
                };

                let on_x_collision =
                    |velocities: &mut XYPair| velocities.x = -velocities.x * object.bounciness();

                let on_y_collision = |velocities: &mut XYPair| {
                    velocities.y = -velocities.y * object.bounciness();

                    // if we're just rolling on the ground, apply ground drag
                    if on_ground && velocities.y.abs() <= 1.0 {
                        velocities.x -= velocities.x * GROUND_DRAG_FACTOR
                    }
                };

                // x axis window collision
                if coords.x <= 0.0 {
                    coords.x = 0.0;
                    on_x_collision(&mut velocities);
                }
                if coords.x + diameter > window_size.width as f64 {
                    coords.x = window_size.width as f64 - diameter;
                    on_x_collision(&mut velocities);
                }

                // y axis window collision
                if coords.y - diameter < 0.0 {
                    coords.y = diameter;
                    on_y_collision(&mut velocities);
                }
                if coords.y + diameter > window_size.height as f64 {
                    coords.y = window_size.height as f64 - diameter;
                    on_y_collision(&mut velocities);
                }

                object.common().coords = coords;
                object.common().velocities = velocities;
            },
            CollisionShape::Rectangle(width, length) => {
                let mut coords = object.common().coords.clone();
                let mut velocities = object.common().velocities.clone();
            
                let on_ground = coords.y + length >= window_size.height as f64;
            
                let on_x_collision = |velocities: &mut XYPair| {
                    velocities.x = -velocities.x * object.bounciness();
                };
            
                let on_y_collision = |velocities: &mut XYPair| {
                    velocities.y = -velocities.y * object.bounciness();
            
                    if on_ground && velocities.y.abs() <= 1.0 {
                        velocities.x -= velocities.x * GROUND_DRAG_FACTOR;
                    }
                };
            
                // x-axis window collision
                if coords.x <= 0.0 {
                    coords.x = 0.0;
                    on_x_collision(&mut velocities);
                } else if coords.x + length > window_size.width as f64 {
                    coords.x = window_size.width as f64 - length;
                    on_x_collision(&mut velocities);
                }
            
                // y-axis window collision
                if coords.y <= 0.0 {
                    coords.y = 0.0;
                    on_y_collision(&mut velocities);
                } else if coords.y + width > window_size.height as f64 {
                    coords.y = window_size.height as f64 - width;
                    on_y_collision(&mut velocities);
                }
            
                object.common().coords = coords;
                object.common().velocities = velocities;
            }            
        }
    }

    fn update_object_info(window_size: &WindowSize, object: &mut Box<dyn GameObject>) {
        object.common().object_info = Some(ObjectInfo {
            window_size: window_size.clone(),
        });
    }

    fn draw(buffer: &mut Vec<u32>, window_size: &WindowSize, object: &mut Box<dyn GameObject>) {
        let raster_vecs = object.draw();

        let common = object.common();
        let coords = &common.coords;

        Engine::draw_at(
            buffer,
            window_size.width,
            window_size.height,
            raster_vecs,
            coords,
        );
    }
}

// internal utils
impl Engine {
    fn draw_at(
        buffer: &mut Vec<u32>,
        buffer_width: usize,
        buffer_height: usize,
        raster_vecs: Vec<Vec<u32>>,
        coords: &XYPair,
    ) {
        let object_width = raster_vecs.iter().map(|row| row.len()).max().unwrap_or(0);

        for (dy, row) in raster_vecs.iter().enumerate() {
            for dx in 0..object_width {
                let x = (coords.x + dx as f64) as usize;
                let y = (coords.y + dy as f64) as usize;

                // make sure this is not out of the buffer
                if x < buffer_width && y < buffer_height {
                    let index = y * buffer_width + x;

                    let maybe_pixel = row.get(dx).cloned();
                    if let Some(pixel) = maybe_pixel {
                        buffer[index] = pixel;
                    }
                }
            }
        }
    }
}

// main run function -- sets up the window and the game loop
impl Engine {
    pub fn run(&mut self, window_title: &str) -> Result<(), anyhow::Error> {
        self.window = Some(Window::new(
            window_title,
            self.window_size.width,
            self.window_size.height,
            WindowOptions {
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )?);

        let duration_per_frame = std::time::Duration::from_secs(1) / FPS.try_into()?;
        
        self.window
            .as_mut()
            .unwrap()
            .limit_update_rate(Some(duration_per_frame));

        while self.window.as_ref().unwrap().is_open()
            && !self.window.as_ref().unwrap().is_key_down(Key::Escape)
        {
            let start_time = std::time::Instant::now();
            let keys = self.window.as_ref().unwrap().get_keys();

            // clear the display buffer
            self.buffer.iter_mut().for_each(|p| *p = 0);

            for object in self.objects.iter_mut() {
                // re-calculate the velocities of the object
                Engine::calc_velocities(object);

                // apply the velocities to the coordinates
                Engine::apply_velocities(object);
                
                // perform collision checks with the window
                Engine::collision_checks(&self.window_size, object);
                                
                // update the game object's info
                Engine::update_object_info(&self.window_size, object);
                
                // allow the object to react to pressed keys
                object.handle_input(&keys);

                // draw the object on the buffer at it's coords
                Engine::draw(&mut self.buffer, &self.window_size, object);
            }
            self.collision_between();

            // reflect the display buffer changes
            self.window.as_mut().unwrap().update_with_buffer(
                &self.buffer,
                self.window_size.width,
                self.window_size.height,
            )?;

            
            //sleep until the next frame is needed.
            std::thread::sleep(
                std::time::Duration::from_secs_f64(DT).saturating_sub(start_time.elapsed()),
            );
        }
        Ok(())
    }
}
