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

pub struct AnimationMode {
    title_frame: TitleFrame,
    menu: AnimationMenu,
    external_input: ExternalInput,
    exit_flag: bool,
    mode_switch_flag: Option<GameModeEnum>,
    game_settings: GameSettings
}

impl AnimationMode {
    pub fn new(game_settings: GameSettings) -> AnimationMode {

        AnimationMode {
            title_frame: AnimationMode::build_title_frame(),
            menu: AnimationMenu::build(game_settings.clone()),
            external_input: Default::default(),
            exit_flag: false,
            mode_switch_flag: None,
            game_settings: game_settings
        }
    }

    pub fn build_title_frame() -> TitleFrame {
        TitleFrame::new("Duel Mode".to_string(), Vector4::new(1.0, 0.0, 0.0, 1.0), Vector2::new(0.0, 0.5), 0.4)
    }

    pub fn menu_action(&mut self, action: AnimationMenuAction) {
        match action {
            AnimationMenuAction::ReturnToMainMenu => self.mode_switch_flag = Some(GameModeEnum::Menu(self.game_settings.clone())),
            AnimationMenuAction::StartAnimation => ()
        }
    }

    // pub fn build_view_matrix(&self) -> ViewDetails2D {
    //     ViewDetails2D {
    //         camera_pos: - (1.0 / self.preview_window.scale) * self.preview_window.pos,
    //         viewport_height: (1.0 / self.preview_window.scale),
    //         viewport_length: (1.0 / self.preview_window.scale),
    //         .. Default::default()
    //     }
    // }
}

impl GameMode for AnimationMode {
    fn update_input(&mut self) {
        self.menu.process_input(&self.external_input);
    }

    fn update_logic(&mut self, t_step: f64) {
        if let Some(action) = self.menu.update(t_step) {
            self.menu_action(action)
        }
    }

    fn update_rendering(&mut self) {}
    fn get_renderables(&self) -> Vec<Box<RenderableTestRenderable>> { 
        let mut output = self.menu.render();
        output.append(&mut self.title_frame.to_renderables());
        output
     }
    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> { 
        Some(&mut self.external_input)
     }
 
    fn change_mode(&self) -> Option<GameModeEnum> { self.mode_switch_flag.clone() }

    fn should_exit(&self) -> bool {
        self.exit_flag
    }

    // fn get_view(&self) -> ViewDetails {
    //     ViewDetails::TwoDim(self.build_view_matrix())
    // }
}

#[derive(Clone, Debug)]
pub enum AnimationMenuAction {
    ReturnToMainMenu,
    StartAnimation
}