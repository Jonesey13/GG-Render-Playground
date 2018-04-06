use gg::input::{MouseInput, KeyboardInput, JoystickInput};
use gg::games::GameInput;

#[derive(Clone, Default)]
pub struct ExternalInput {
    pub mouse: MouseInput,
    pub kbd: KeyboardInput,
    pub joystick: JoystickInput
}

impl GameInput for ExternalInput {
    fn get_mouse_inp<'a>(&'a mut self) -> Option<&'a mut MouseInput> { Some(&mut self.mouse) }
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
    fn get_joystick_inp<'a>(&'a mut self) -> Option<&'a mut JoystickInput> { Some(&mut self.joystick) }
}