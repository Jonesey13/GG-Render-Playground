use ::menu::{MenuOption, Menu, MenuClickable, MenuRange, MenuEnum, MenuInput};
use ::common::GameSettings;
use super::AnimationMenuAction;
use ::animation::Animatable;
use na::{Vector2, Vector3};

pub struct AnimationMenu {
    normal_options: Vec<MenuClickable<AnimationMenuAction>>,
    current_selection: usize,
    pub animation_select_option: MenuEnum<String, AnimationMenuAction>,
    input: MenuInput
}

impl AnimationMenu {
    pub fn build(settings: GameSettings, animations: &Vec<Box<Animatable>>) -> Self {
        let mut normal_options: Vec<MenuClickable<AnimationMenuAction>> = Vec::new();
        normal_options.push(MenuClickable::new(
                0,
                "Start Animation".to_string(), 
                Vector3::new(-0.8, 0.0, 0.0), 
                0.1, 
                AnimationMenuAction::StartAnimation
            ));
        normal_options.push(MenuClickable::new(
                2,
                "Return to Main Menu".to_string(), 
                Vector3::new(-0.8, -0.3, 0.0), 
                0.1, 
                AnimationMenuAction::ReturnToMainMenu
        ));

        let list_of_animations: Vec<String> = animations.iter().map(|anim| {anim.get_name()}).collect();

        let animation_select_option = MenuEnum::<String, AnimationMenuAction>::new(
            1,
            "Animation".to_string(),
            Vector3::new(-0.8, -0.1, 0.0),
            0.1,
            list_of_animations[0].clone(),
            list_of_animations.clone(),
            list_of_animations
        );

        Self {
            normal_options,
            input: Default::default(),
            animation_select_option,
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

        options.push(&self.animation_select_option);
        options
    }

    fn get_options_mut(&mut self) -> Vec<&mut MenuOption<Action=Self::Action>> { 
        let mut options: Vec<&mut MenuOption<Action=Self::Action>> = 
            self.normal_options
            .iter_mut()
            .map(|opt| -> &mut MenuOption<Action=Self::Action> {opt})
            .collect();

        options.push(&mut self.animation_select_option);
        options
     }

    fn get_input_mut(&mut self) -> &mut MenuInput { &mut self.input }

    fn get_current_selection(&self) -> usize {self.current_selection}

    fn set_current_selection(&mut self, index: usize) {self.current_selection = index;}
}
    
