use crate::{
    fbs_utils::BahaiControl,
    mahjong_generated::open_mahjong::{Bahai, GameStateT},
};

impl GameStateT {
    pub fn create(&mut self, title: &[u8], player_len: u32) {
        self.player_len = player_len;
    }

    pub fn start(&mut self) {
        let bahai = Bahai::create_shuffled();

        self.bahai = bahai.unpack();
    }
}
