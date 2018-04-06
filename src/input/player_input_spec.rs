#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerInputSpec {
    Unbound,
    Keyboard(KeyboardInputSpec),
    Joystick(usize),
    Mouse(usize)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeyboardInputSpec {
    WASD,
    UpDownLeftRight
}