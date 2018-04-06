use ::menu::{MenuOption, Menu, MenuClickable, MenuRange, MenuEnum, MenuInput};
use ::common::GameSettings;
use super::AnimationMenuAction;
use na::{Vector2, Vector3};

pub struct AnimationMenu {
    normal_options: Vec<MenuClickable<AnimationMenuAction>>,
    current_selection: usize,
    input: MenuInput
}

impl AnimationMenu {
    pub fn build(settings: GameSettings) -> Self {
        let mut normal_options: Vec<MenuClickable<AnimationMenuAction>> = Vec::new();
        normal_options.push(MenuClickable::new(
                0,
                "Start Animation".to_string(), 
                Vector3::new(-0.5, 0.0, 0.0), 
                0.1, 
                AnimationMenuAction::StartAnimation
            ));
        normal_options.push(MenuClickable::new(
                1,
                "Return to Main Menu".to_string(), 
                Vector3::new(-0.5, -0.3, 0.0), 
                0.1, 
                AnimationMenuAction::ReturnToMainMenu
        ));

        Self {
            normal_options,
            input: Default::default(),
            current_selection: 0
        }
    }
}

impl Menu for AnimationMenu {
    type Action = AnimationMenuAction;

    fn get_options(&self) -> Vec<&MenuOption<Action=Self::Action>> { 
        let mut options: Vec<&MenuOption<Action=Self::Action>> = 
            self.normal_options
            .iter()
            .map(|opt| -> &MenuOption<Action=Self::Action> {opt})
            .collect();

        options
    }

    fn get_options_mut(&mut self) -> Vec<&mut MenuOption<Action=Self::Action>> { 
        let mut options: Vec<&mut MenuOption<Action=Self::Action>> = 
            self.normal_options
            .iter_mut()
            .map(|opt| -> &mut MenuOption<Action=Self::Action> {opt})
            .collect();

        options
     }

    fn get_input_mut(&mut self) -> &mut MenuInput { &mut self.input }

    fn get_current_selection(&self) -> usize {self.current_selection}

    fn set_current_selection(&mut self, index: usize) {self.current_selection = index;}
}
    
