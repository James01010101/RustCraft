
/* this will store all the data related to key pressed
what keys are currently being held down and so on
*/

pub struct MyKeyboard {
    // keep track if the wasd keys for movement and if they are being held down
    pub w_held: bool,
    pub a_held: bool,
    pub s_held: bool,
    pub d_held: bool,
}


impl MyKeyboard {
    pub fn new() -> MyKeyboard {
        MyKeyboard {
            w_held: false,
            a_held: false,
            s_held: false,
            d_held: false,
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
}