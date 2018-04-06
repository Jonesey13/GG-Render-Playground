use super::{MenuOption, MenuClickable, MenuInput, MenuBackground, MenuTitle, Menu};

pub struct StandardMenu<A> {
    pub options: Vec<MenuClickable<A>>,
    current_selection: usize,
    menu_input: MenuInput,
    background: Option<MenuBackground>,
    title: Option<MenuTitle>
}

impl<A: Clone> StandardMenu<A> {
    pub fn new(options: Vec<MenuClickable<A>>) -> Self {
        Self {
            options,
            current_selection: 0,
            menu_input: Default::default(),
            background: None,
            title: None
        }
    }

    pub fn new_with_background(options: Vec<MenuClickable<A>>, background: MenuBackground) -> Self {
        Self {
            options,
            current_selection: 0,
            menu_input: Default::default(),
            background: Some(background),
            title: None
        }
    }

    pub fn new_with_title_and_background(options: Vec<MenuClickable<A>>, title: MenuTitle, background: MenuBackground) -> Self {
        Self {
            options,
            current_selection: 0,
            menu_input: Default::default(),
            background: Some(background),
            title: Some(title)
        }
    }
}

impl<A: Clone> Menu for StandardMenu<A> {
    type Action = A;

    fn get_options(&self) -> Vec<&MenuOption<Action=Self::Action>> { 
        self.options.iter().map(|opt| -> &MenuOption<Action=Self::Action> {opt}).collect()
    }

    fn get_options_mut(&mut self) -> Vec<&mut MenuOption<Action=Self::Action>> { 
        self.options.iter_mut().map(|opt| -> &mut MenuOption<Action=Self::Action> {opt}).collect()
    }

    fn get_input_mut(&mut self) -> &mut MenuInput {&mut self.menu_input}

    fn get_current_selection(&self) -> usize {self.current_selection}

    fn set_current_selection(&mut self, index: usize) {self.current_selection = index}

    fn get_background(&self) -> Option<&MenuBackground> { self.background.as_ref() }

    fn get_title(&self) -> Option<&MenuTitle> { self.title.as_ref() }
}