mod animation_menu;

use ::game::{GameMode, GameModeEnum};
use ::common::{GameSettings, TitleFrame};
use ::input::ExternalInput;
use ::menu::{MenuEnum, Menu};
use gg::games::GameInput;
use gg::games::view_details::{ViewDetails, ViewDetails2D};
use ::rendering::RenderableTestRenderable;
use na::{Vector2, Vector4};
use ::animation_test_mode::animation_menu::AnimationMenu;
use ::constants;
use ::animation::Animatable;
use ::rendering::animations::*;

pub struct AnimationMode {
    title_frame: TitleFrame,
    menu: AnimationMenu,
    external_input: ExternalInput,
    exit_flag: bool,
    mode_switch_flag: Option<GameModeEnum>,
    game_settings: GameSettings,
    animations: Vec<Box<Animatable>>,
    running_animation: Option<String>
}

impl AnimationMode {
    pub fn new(game_settings: GameSettings) -> AnimationMode {
        let animations: Vec<Box<Animatable>> = vec![
            Box::new(PongStretch::new()),
            Box::new(PongFlip::new())
        ];

        AnimationMode {
            title_frame: AnimationMode::build_title_frame(),
            menu: AnimationMenu::build(game_settings.clone(), &animations),
            external_input: Default::default(),
            exit_flag: false,
            mode_switch_flag: None,
            game_settings: game_settings,
            animations,
            running_animation: None
        }
    }

    pub fn build_title_frame() -> TitleFrame {
        TitleFrame::new("Animations List".to_string(), Vector4::new(1.0, 0.0, 0.0, 1.0), Vector2::new(0.0, 0.8), 0.4)
    }

    pub fn menu_action(&mut self, action: AnimationMenuAction) {
        match action {
            AnimationMenuAction::ReturnToMainMenu => self.mode_switch_flag = Some(GameModeEnum::Menu(self.game_settings.clone())),
            AnimationMenuAction::StartAnimation => {
                if let Some(ref mut animation) = self.get_running_animation_mut() {
                    animation.reset()
                }
                self.running_animation = Some(self.menu.animation_select_option.get_value().clone());
            }
        }
    }

    pub fn get_running_animation(&self) -> Option<&Box<Animatable>> {
        self.running_animation.clone().and_then(|anim_name| {
            self.animations.iter().find(|anim| {anim.get_name() == anim_name})
        })
    }

    pub fn get_running_animation_mut(&mut self) -> Option<&mut Box<Animatable>> {
        self.running_animation.clone().and_then(move |anim_name| {
            self.animations.iter_mut().find(|anim| {anim.get_name() == anim_name})
        })
    } 

    pub fn build_view_matrix(&self) -> ViewDetails2D {
        ViewDetails2D {
            camera_pos: Vector2::new(-0.45, 0.15),
            .. Default::default()
        }
    }
}

impl GameMode for AnimationMode {
    fn update_input(&mut self) {
        self.menu.process_input(&self.external_input);
    }

    fn update_logic(&mut self, t_step: f64) {
        if let Some(action) = self.menu.update(t_step) {
            self.menu_action(action)
        }

        if let Some(ref mut animation) = self.get_running_animation_mut() {
            animation.update(t_step);
        }
    }

    fn update_rendering(&mut self) {
    }

    fn get_renderables(&self) -> Vec<Box<RenderableTestRenderable>> { 
        let mut output = self.menu.render();
        output.append(&mut self.title_frame.to_renderables());
        if let Some(ref animatable) = self.get_running_animation() {
            output.append(&mut animatable.render());
        }
        output
     }
    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> { 
        Some(&mut self.external_input)
     }
 
    fn change_mode(&self) -> Option<GameModeEnum> { self.mode_switch_flag.clone() }

    fn should_exit(&self) -> bool {
        self.exit_flag
    }

    fn get_view(&self) -> ViewDetails {
        ViewDetails::TwoDim(self.build_view_matrix())
    }
}

#[derive(Clone, Debug)]
pub enum AnimationMenuAction {
    ReturnToMainMenu,
    StartAnimation
}