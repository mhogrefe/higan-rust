//TODO test

#[derive(Debug, PartialEq, Eq)]
pub enum Model {
    GameBoy,
    GameBoyColor,
    SuperGameBoy,
}

impl Default for Model {
    fn default() -> Model {
        Model::GameBoy
    }
}

#[derive(Debug, Default)]
pub struct System {
    pub model: Model,
}

impl System {
    pub fn model_is_game_boy(&self) -> bool {
        self.model == Model::GameBoy
    }

    pub fn model_is_game_boy_color(&self) -> bool {
        self.model == Model::GameBoyColor
    }

    pub fn model_is_super_game_boy(&self) -> bool {
        self.model == Model::SuperGameBoy
    }
}
