use super::{PlayerInputSpec, KeyboardInputSpec};

#[derive(Clone, Debug)]
pub struct InputSettings {
    pub players: Vec<PlayerInputSpec>
}

impl InputSettings {
    pub fn clear(&mut self) {
        for i in 0..self.players.len() {
            self.players[i] = PlayerInputSpec::Unbound;
        }
    }

    pub fn new(players: Vec<PlayerInputSpec>) -> Self {
        Self {
            players
        }
    }

    pub fn get_player_setting(&self, index: usize) -> PlayerInputSpec {
        self.players.iter().cloned().nth(index).expect("Missing Player Spec!")
    }
}

impl Default for InputSettings {
    fn default() -> Self {

        Self::new(vec![
            PlayerInputSpec::Unbound,
            PlayerInputSpec::Unbound,
            PlayerInputSpec::Unbound,            
            PlayerInputSpec::Unbound,
            PlayerInputSpec::Unbound,
            PlayerInputSpec::Unbound,
            PlayerInputSpec::Unbound,
            PlayerInputSpec::Unbound
            ])
    }
}