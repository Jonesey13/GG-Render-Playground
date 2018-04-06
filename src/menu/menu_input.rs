use gg::input::bool_switch::BoolSwitch;
use ::input::ExternalInput;
use ::constants;

#[derive(Default, Clone)]
pub struct MenuInput {
    pub up_switch: BoolSwitch,
    pub down_switch: BoolSwitch,
    pub left_switch: BoolSwitch,
    pub right_switch: BoolSwitch,
    pub select_switch: BoolSwitch
}

impl MenuInput {
    pub fn update_switches(&mut self, external_input: &ExternalInput) {
        self.up_switch.update_state(external_input.kbd.get_up() 
            || external_input.joystick.get_y_axis_right_flag(constants::input::JOYSTICK_MENU_DEADZONE));
        self.down_switch.update_state(external_input.kbd.get_down()
            || external_input.joystick.get_y_axis_left_flag(constants::input::JOYSTICK_MENU_DEADZONE));
        self.left_switch.update_state(external_input.kbd.get_left());
        self.right_switch.update_state(external_input.kbd.get_right());
        self.select_switch.update_state(external_input.kbd.get_ret() || external_input.joystick.get_button_1());        
    }

    pub fn clear_switches(&mut self) {
        self.up_switch.clear_switch();
        self.down_switch.clear_switch();
        self.left_switch.clear_switch();
        self.right_switch.clear_switch();
        self.select_switch.clear_switch();
    }
}