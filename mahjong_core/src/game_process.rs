use crate::{
    fbs_utils::TakuControl,
    mahjong_generated::open_mahjong::{Taku, GameStateT, PlayerT, PaiT, ActionType, TakuT}, shanten::PaiState,
};

impl GameStateT {
    pub fn create(&mut self, title: &[u8], player_len: u32) {
        self.player_len = player_len;

        for idx in 0..self.player_len {
            let player = &mut self.players[idx as usize];
            player.score = 25000;
        }
    }

    pub fn start(&mut self) {
        self.taku = TakuT::create_shuffled();

        // 配牌
        for idx in 0..self.player_len {
            let player = &mut self.players[idx as usize];
            let cursol = self.taku_cursol as usize;
            let r = self.taku.get_range(cursol..(cursol+13));

            if let Ok(mut v) = r {
                v.sort_unstable();
                for (i, item) in v.into_iter().enumerate() {
                    player.tehai[i] = item;
                }
                player.tehai_len = 13;
            }

            self.taku_cursol += 13;
        }
    }

    pub fn get_player(&self, index: usize) -> PlayerT {
        self.players[index].clone()
    }

    pub fn tsumo(&mut self) -> Result<(), ()> {
        let player = &mut self.players[self.teban as usize];
        player.is_tsumo = true;
        player.tsumohai = self.taku.get(self.taku_cursol as usize)?;
        self.taku_cursol += 1;

        Ok(())
    }

    pub fn sutehai(&mut self, index: usize) {
        let player = &mut self.players[self.teban as usize];
        let mut tehai: Vec<PaiT> = player.tehai.iter().cloned().collect();

        if index != 14 {
            tehai.remove(index);
            tehai.push(player.tsumohai.clone());
            tehai.sort_unstable();

            for (i, item) in tehai.into_iter().enumerate() {
                player.tehai[i] = item;
            }
        }
        player.tsumohai = Default::default();

        player.is_tsumo = false;
        self.teban += 1;
        if self.teban == self.player_len {
            self.teban = 0;
        }
    }

    pub fn tsumo_agari(&mut self) -> Result<(), ()> {
        let player = &mut self.players[self.teban as usize];
        let mut tehai: Vec<PaiT> = player.tehai.iter().cloned().collect();

        tehai.push(player.tsumohai.clone());

        let mut state = PaiState::from(&tehai);

        // 上がり判定
        if state.get_shanten() == -1 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn action(&mut self, action_type: ActionType, player_index: usize, param: u64) -> Result<(), ()> {
        match action_type {
            ActionType::ACTION_SYNC => {
                if player_index == self.teban as usize {
                    self.tsumo()
                } else {
                    Ok(())
                }
            },
            ActionType::ACTION_SUTEHAI => {
                if player_index == self.teban as usize {
                    self.sutehai(param as usize);
                    Ok(())
                } else {
                    Err(())
                }
            },
            ActionType::ACTION_CHII => todo!(),
            ActionType::ACTION_PON => todo!(),
            ActionType::ACTION_KAN => todo!(),
            ActionType::ACTION_TSUMO => {
                if player_index == self.teban as usize {
                    self.tsumo_agari()
                } else {
                    Err(())
                }
            },
            ActionType::ACTION_NAGASHI => todo!(),
            _ => Err(()),
        }
    }
}
