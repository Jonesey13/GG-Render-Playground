use ::input::InputSettings;
use gg::rendering::DisplaySettings;

#[derive(Clone, Debug)]
pub struct GameSettings {
    pub input_settings: InputSettings,
    pub display_settings: DisplaySettings,
}

impl GameSettings {
    pub fn new_with_display_settings(display_settings: DisplaySettings) -> Self {
        Self {
            input_settings: Default::default(),
            display_settings
        }
    }
}