use crate::{my_keyboard::*, types::*};

pub struct Character {
    // current position im standing at (where my head is)
    pub position: FPosition,

    // the position im looking at (for camera rendering stuff)
    pub target: FPosition,

    // mouse movement
    pub yaw: f32,   // x axis
    pub pitch: f32, // y axis

    // chunk im standing in
    pub chunk_position: (i32, i32),

    // if i have changed the chunk im standing in (so i can load new chunks in and out)
    pub chunk_changed: bool,

    // settings
    pub movement_speed: f32,
}

impl Character {
    pub fn new(movement_speed: f32) -> Character {
        Character {
            position: FPosition {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            target: FPosition {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            chunk_position: (0, 0),
            yaw: 0.0,
            pitch: 0.0,
            chunk_changed: true, // init to true to it loads in the correct chunks
            movement_speed,
        }
    }

    // calculate the chunk the player is in from its position
    pub fn update_chunk_position(&mut self, chunk_sizes: (usize, usize, usize)) {
        // this does the divide thing but if negative works properly

        let new_chunk_x: i32 = self.position.x.div_euclid(chunk_sizes.0 as f32) as i32;
        let new_chunk_z: i32 = self.position.z.div_euclid(chunk_sizes.2 as f32) as i32;

        // check if these are not the same as last frame
        if new_chunk_x != self.chunk_position.0 || new_chunk_z != self.chunk_position.1 {
            self.chunk_position = (new_chunk_x, new_chunk_z);
            self.chunk_changed = true;
        }
    }

    pub fn get_current_chunk(&self) -> (i32, i32) {
        self.chunk_position
    }

    pub fn move_forward(&mut self, amount: f32) {
        // move forward in the direction im facing
        self.position.x += self.yaw.cos() * amount;
        self.position.z += self.yaw.sin() * amount;
    }

    pub fn move_sideways(&mut self, amount: f32) {
        // move sideways in the direction im facing (1.57 is roughly pi/2 or 90 degrees)
        self.position.x += (self.yaw + 1.57).cos() * amount;
        self.position.z += (self.yaw + 1.57).sin() * amount;
    }

    pub fn update_view(&mut self, keyboard: &mut MyKeyboard) {
        // the variables are differences from their positions last frame
        let mut x_change: f32 = keyboard.mouse_position.0 - keyboard.mouse_center_position.0;
        let mut y_change: f32 = keyboard.mouse_position.1 - keyboard.mouse_center_position.1;

        // only update the camera if its moved more than a threshold.
        // if it is less than the threshold then itll be a small movement and i dont want to update the camera for that
        let threshold: f32 = 5.0;

        // get the distance between the mouse position and then center of the screen
        let distance: f32 = ((x_change * x_change) + (y_change * y_change)).sqrt();

        if distance < threshold {
            x_change = 0.0;
            y_change = 0.0;
        }

        // Update yaw and pitch based on mouse movement
        self.yaw += x_change * keyboard.mouse_sensitivity_h;
        self.pitch -= y_change * keyboard.mouse_sensitivity_v;

        // Clamp pitch to prevent looking too far up or down
        self.pitch = self.pitch.clamp(-1.57, 1.57);

        // Update target position based on new direction
        self.target.x = self.position.x + (self.yaw.cos() * self.pitch.cos());
        self.target.y = self.position.y + self.pitch.sin();
        self.target.z = self.position.z + (self.yaw.sin() * self.pitch.cos());
    }

    //TODO: #110 save character position on cleanup and load it back in on load
}
