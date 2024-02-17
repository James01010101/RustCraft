
/* this will store all the data related to key pressed
what keys are currently being held down and so on
*/

pub struct MyKeyboard {

    // the current position of the mouse (updated every time the mouse moves so i know its position)
    pub mouse_position: (f32, f32),

    // the middle of the screen that the mouse will return to each frame
    pub mouse_center_position: (f32, f32),



    // keep track if the wasd keys for movement and if they are being held down
    pub w_held: bool,
    pub a_held: bool,
    pub s_held: bool,
    pub d_held: bool,
}


impl MyKeyboard {
    pub fn new(screen_center: (f32, f32)) -> MyKeyboard {
        MyKeyboard {
            w_held: false,
            a_held: false,
            s_held: false,
            d_held: false,

            mouse_position: (0.0, 0.0),
            mouse_center_position: screen_center,
        }
    }


    // pressed buttons
    pub fn pressed_w(&mut self) {
        self.w_held = true;
    }

    pub fn pressed_a(&mut self) {
        self.a_held = true;
    }

    pub fn pressed_s(&mut self) {
        self.s_held = true;
    }

    pub fn pressed_d(&mut self) {
        self.d_held = true;
    }


    // released buttons
    pub fn released_w(&mut self) {
        self.w_held = false;
    }

    pub fn released_a(&mut self) {
        self.a_held = false;
    }

    pub fn released_s(&mut self) {
        self.s_held = false;
    }

    pub fn released_d(&mut self) {
        self.d_held = false;
    }


    // each resize of the window i need to update the center of the screen
    pub fn update_screen_center(&mut self, new_x: f32, new_y: f32) {
        self.mouse_center_position = (new_x, new_y);
    }

    // update mouse position
    pub fn update_mouse_position(&mut self, new_x: f32, new_y: f32) {
        self.mouse_position = (new_x, new_y);

    }
}