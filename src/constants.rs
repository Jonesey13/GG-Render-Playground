pub mod window {
    //pub const ASPECT_RATIO: f64 = 16.0 / 9.0; 
    //pub const VIEWPORT_HEIGHT: f64 = 2.0;
    //pub const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
}

pub mod input {
    pub const JOYSTICK_DEADZONE: f64 = 0.2;
    pub const JOYSTICK_MENU_DEADZONE: f64 = 0.5;
}

pub mod menu {
    pub const SELECTED_COLOUR: [f64; 4] = [0.0, 1.0, 0.0, 1.0];
    pub const UNSELECTED_COLOUR: [f64; 4] = [1.0, 1.0, 1.0, 1.0];
    //pub const BACKGROUND_COLOUR: [f64; 4] = [0.025, 0.075, 0.15, 1.0];
    //pub const BACKGROUND_DEPTH: f64 = 0.8;
}

pub mod pong_stretch {
    pub const ORIGINAL_DIM: [f64; 2] = [0.1, 0.5];
    pub const STRETCH_DIM: [f64; 2] = [0.1, 1.0];
}

pub mod pong_flip {
    pub const DIM: [f64; 2] = [0.1, 0.5];
    pub const FLIP_BACK_DIST: f64 = 0.15;
    pub const FLIP_BACK_TIME: f64 = 0.2;
    pub const FLIP_DURATION: f64 = 0.25;
}

pub mod pong {
    pub const PADDLE_COLOUR: [f64; 4] = [0.2, 0.2, 1.0, 1.0];
}