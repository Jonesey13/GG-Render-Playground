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
    pub const FLIP_DURATION: f64 = 0.35;
    pub const FLIP_FULCRUM_POS: [f64; 2] = [0.0, -0.125];
}

pub mod pong_grab {
    pub const DIM: [f64; 2] = [0.1, 0.5];
    pub const GRAB_PRE_RADIUS: f64 = 0.2;
    pub const BALL_START_POS: [f64; 2] = [1.0, 0.0];
}

pub mod ball {
    pub const RADIUS: f64 = 0.1;
    pub const BALL_COLOUR: [f64; 4] = [1.0, 1.0, 1.0, 1.0];
}

pub mod pong {
    pub const PADDLE_COLOUR: [f64; 4] = [0.2, 0.2, 1.0, 1.0];
}

pub mod whack_item {
    pub const DEFAULT_SIZE: f64 = 0.5;
    pub const THICKNESS: f64 = 0.05;
    pub const ARROW_ANGLE_DIM: [f64; 2] = [0.2, 1.0];
    pub const ICON_THICKNESS: f64 = 0.075;
    pub const ARROW_DIM: [f64; 2] = [0.1, 0.15];
    pub const SPIKE_SIZE: f64 = 0.1;
    pub const NUM_SPIKES: usize = 10;
    pub const ICON_SCALE: f64 = 0.65;
    pub const BACKGROUND_COLOUR: [f64; 4] = [1.0, 0.4, 0.4, 1.0];
    pub const FOREGROUND_COLOUR: [f64; 4] = [1.0, 1.0, 1.0, 1.0];
}

pub mod stretch_item {
    pub const DEFAULT_SIZE: f64 = 0.5;
    pub const RECT_WIDTH: f64 = 0.4;
    pub const THICKNESS: f64 = 0.05;
    pub const ARROW_THICKNESS: f64 = 0.1;
    pub const ARROW_DIM: [f64; 2] = [0.1, 0.15];
    pub const ARROW_SCALE: f64 = 0.7;
    pub const BACKGROUND_COLOUR: [f64; 4] = [0.4, 0.4, 1.0, 1.0];
    pub const FOREGROUND_COLOUR: [f64; 4] = [1.0, 1.0, 1.0, 1.0];
}

pub mod grab_item {
    pub const DEFAULT_SIZE: f64 = 0.5;
    pub const PADDLE_DIM: [f64; 2] = [0.14, 0.7];
    pub const PADDLE_POS: [f64; 2] = [-0.1, 0.0];
    pub const GRAB_RADIUS: f64 = 0.5;
    pub const GRAB_ANGLES: [f64; 2] = [0.125, 0.875];
    pub const THICKNESS: f64 = 0.05;
    pub const BACKGROUND_COLOUR: [f64; 4] = [0.1, 0.8, 0.4, 1.0];
    pub const FOREGROUND_COLOUR: [f64; 4] = [1.0, 1.0, 1.0, 1.0];
}