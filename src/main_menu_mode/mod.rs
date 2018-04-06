use ::game::{GameMode, GameModeEnum};
use ::common::{TitleFrame, GameSettings};
use ::menu::{Menu, MenuClickable, StandardMenu};
use ::input::ExternalInput;
use gg::games::GameInput;
use ::rendering::{RenderableTestRenderable};
use na::{Vector2, Vector3, Vector4};
use ::renderable_test_mode::RenderableTestMode;

pub struct MainMenuMode {
    title_frame: TitleFrame,
    menu: StandardMenu<MainMenuAction>,
    external_input: ExternalInput,
    exit_flag: bool,
    mode_switch_flag: Option<GameModeEnum>,
    game_settings: GameSettings
}

impl MainMenuMode {
    pub fn new(game_settings: GameSettings) -> MainMenuMode { 
        MainMenuMode {
            title_frame: Self::build_title_frame(),
            menu: MainMenuMode::build_menu(),
            external_input: Default::default(),
            exit_flag: false,
            mode_switch_flag: None,
            game_settings
        }
    }

    pub fn build_menu() -> StandardMenu<MainMenuAction> {
        let mut options: Vec<MenuClickable<MainMenuAction>> = Vec::new();
        options.push(
            MenuClickable::new(
                0,
                "Renderable Test".to_string(), 
                Vector3::new(0.0, -0.15, 0.0), 
                0.1, 
                MainMenuAction::RenderableTest
            ));
        options.push(
            MenuClickable::new(
                1,
                "Animations".to_string(), 
                Vector3::new(0.0, -0.3, 0.0), 
                0.1, 
                MainMenuAction::AnimationMenu
            ));
        options.push(
            MenuClickable::new(
                2,
                "Exit".to_string(), 
                Vector3::new(0.0, -0.45, 0.0), 
                0.1, 
                MainMenuAction::Exit
            ));

        StandardMenu::new(options)
    }

    pub fn build_title_frame() -> TitleFrame {
        TitleFrame::new("Render Test Mode".to_string(), Vector4::new(1.0, 1.0, 1.0, 1.0), Vector2::new(0.0, 0.85), 0.3)
    }

    pub fn menu_action(&mut self, action: MainMenuAction) {
        match action {
            MainMenuAction::Exit => self.exit_flag = true,
            MainMenuAction::RenderableTest => self.mode_switch_flag = Some(GameModeEnum::RenderableTest(self.game_settings.clone())),
            MainMenuAction::AnimationMenu => self.mode_switch_flag = Some(GameModeEnum::AnimationMenu(self.game_settings.clone()))
        }
    }
}

impl GameMode for MainMenuMode {
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
}

#[derive(Clone, Debug)]
pub enum MainMenuAction {
    Exit,
    RenderableTest,
    AnimationMenu
}