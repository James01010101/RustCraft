
use crate::{
    types::*,
    settings::*,
    my_keyboard::*,
};

pub struct Character {

    // current position im standing at (where my head is)
    pub position: FPosition,

    // the position im looking at (for camera rendering stuff)
    pub target: FPosition,

    // mouse movement
    pub yaw: f32, // x axis
    pub pitch: f32, // y axis

    // chunk im standing in
    pub chunk_position: (i32, i32),
}


impl Character {
    pub fn new() -> Character {
        Character {
            position: FPosition { x: 0.0, y: 2.0, z: -5.0 },
            target: FPosition { x: 0.0, y: 0.0, z: 0.0 },
            chunk_position: (0, 0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn update_chunk_position(&mut self) {
        self.chunk_position = (self.position.x as i32 / CHUNK_SIZE_X as i32, self.position.z as i32 / CHUNK_SIZE_Z as i32);
    }

    pub fn get_current_chunk(&self) -> (i32, i32) {
        self.chunk_position
    }

    pub fn move_forward(&mut self, amount: f32) {
        self.position.z += amount;

        // the move target is right. so if i dont move the mouse itll keep where it was but move along with the position.
        // so it wont lock onto a block as i move but will slide which is what i want
        self.target.z += amount;
    }

    pub fn move_sideways(&mut self, amount: f32) {
        self.position.x += amount;
        self.target.x += amount;
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
        self.yaw += x_change * MOUSE_SENSITIVITY;
        self.pitch -= y_change * MOUSE_SENSITIVITY;

        // Clamp pitch to prevent looking too far up or down
        self.pitch = self.pitch.clamp(-1.57, 1.57);

        // Update target position based on new direction
        self.target.x = self.position.x + (self.yaw.cos() * self.pitch.cos());
        self.target.y = self.position.y + self.pitch.sin();
        self.target.z = self.position.z + (self.yaw.sin() * self.pitch.cos());
        
    }
    

    //TODO: #110 save character position on cleanup and load it back in on load
}