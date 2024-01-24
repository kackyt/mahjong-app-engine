use crate::{
    fbs_utils::TakuControl,
    mahjong_generated::open_mahjong::{GameStateT, PlayerT, PaiT, ActionType, TakuT}, shanten::PaiState,
};

const DORA_START_INDEX : usize = 0;
const URADORA_START_INDEX : usize = 5;

impl GameStateT {
    pub fn create(&mut self, title: &[u8], player_len: u32) {
        self.player_len = player_len;

        for idx in 0..self.player_len {
            let player = &mut self.players[idx as usize];
            player.score = 25000;
        }
    }

    pub fn shuffle(&mut self) {
        self.taku = TakuT::create_shuffled()
    }

    pub fn load(&mut self, hai_ids: &Vec<u32>) {
        self.taku = TakuT::load(hai_ids);
    }

    pub fn next_cursol(&mut self) {
        if self.is_duplicate_mode {
            self.players[self.teban as usize].cursol += 1;
        } else {
            self.taku_cursol += 1;
        }
    }

    pub fn get_zikaze(&self, who: usize) -> u32 {
        let diff = (who as i32) - (self.oya as i32);

        if diff < 0 {
            (diff + self.player_len as i32) as u32
        } else {
            diff as u32
        }
    }

    pub fn start(&mut self) {
        // 配牌
        self.taku_cursol = 14;
        self.dora_len = 1;
        self.uradora_len = 0;

        for idx in 0..self.player_len {
            let player = &mut self.players[idx as usize];
            let cursol: &mut u32;

            player.cursol = 14 + (idx * if idx < 2 { 31 } else { 30 });

            if self.is_duplicate_mode {
                cursol = &mut player.cursol;
            } else {
                cursol = &mut self.taku_cursol;
            }
            let r = self.taku.get_range((*cursol as usize)..(*cursol+13) as usize);

            if let Ok(mut v) = r {
                v.sort_unstable();
                for (i, item) in v.into_iter().enumerate() {
                    player.tehai[i] = item;
                }
                player.tehai_len = 13;
            }

            *cursol += 13;
        }
    }

    pub fn get_player(&self, index: usize) -> PlayerT {
        self.players[index].clone()
    }

    pub fn tsumo(&mut self) -> Result<(), ()> {
        let player = &mut self.players[self.teban as usize];
        player.is_tsumo = true;

        if self.is_duplicate_mode {
            player.tsumohai = self.taku.get(player.cursol as usize)?;
        } else {
            player.tsumohai = self.taku.get(self.taku_cursol as usize)?;
        }

        self.next_cursol();

        Ok(())
    }

    pub fn sutehai(&mut self, index: usize) {
        let player = &mut self.players[self.teban as usize];
        let mut tehai: Vec<PaiT> = player.tehai.iter().cloned().collect();

        if index != 13 {
            let kawahai = tehai.remove(index);
            tehai.push(player.tsumohai.clone());
            tehai.sort_unstable();

            for (i, item) in tehai.into_iter().enumerate() {
                player.tehai[i] = item;
            }
            player.kawahai[player.kawahai_len as usize] = kawahai;
        } else {
            player.kawahai[player.kawahai_len as usize] = player.tsumohai.clone();
        }
        player.kawahai_len += 1;
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
        if state.get_shanten(0) == -1 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn action(&mut self, action_type: ActionType, player_index: usize, param: u32) -> Result<(), ()> {
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

    pub fn copy_dora(&mut self, dora: &Vec<PaiT>) {
        self.dora_len = dora.len() as u32;
        for (i, item) in dora.iter().enumerate() {
            self.taku.n1[(DORA_START_INDEX + i) as usize] = item.clone();
        }
    }

    pub fn copy_uradora(&mut self, uradora: &Vec<PaiT>) {
        self.uradora_len = uradora.len() as u32;
        for (i, item) in uradora.iter().enumerate() {
            self.taku.n1[(URADORA_START_INDEX + i) as usize] = item.clone();
        }
    }

    pub fn get_dora(&self) -> &[PaiT] {
        &self.taku.n1[DORA_START_INDEX..(DORA_START_INDEX + self.dora_len as usize)]
    }

    pub fn get_uradora(&self) -> &[PaiT] {
        &self.taku.n1[URADORA_START_INDEX..(URADORA_START_INDEX + self.uradora_len as usize)]
    }
}
