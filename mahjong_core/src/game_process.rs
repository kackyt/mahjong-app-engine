use crate::{
    agari::{add_machi_to_mentsu, Agari, AgariBehavior},
    fbs_utils::TakuControl,
    mahjong_generated::open_mahjong::{ActionType, GameStateT, PaiT, PlayerT, RuleT, TakuT},
    play_log::PlayLog,
    shanten::{all_of_mentsu, PaiState},
};
use anyhow::{bail, ensure};
use chrono::Utc;
use itertools::Itertools;
use uuid::Uuid;

const DORA_START_INDEX: usize = 0;
const URADORA_START_INDEX: usize = 5;

impl RuleT {
    pub fn update_to_default(&mut self) {
        self.enable_kuitan = true;
        self.enable_kansaki = false;
        self.enable_pao = false;
        self.initial_score = 25000;
        self.enable_tobi = true;
        self.enable_wareme = false;
        self.aka_type = 0;
        self.shanyu_score = 0;
        self.nannyu_score = -1;
        self.enable_kuinaoshi = true;
        self.uradora_type = 2;
        self.enable_minus_riichi = true;
        self.enable_ryanhan_shibari = false;
        self.enable_keiten = true;
        self.oyanagare_type = 0x0f;
        self.kan_in_riichi = 1;
        self.enable_kiriage = false;
    }
}

impl GameStateT {
    pub fn create(&mut self, title: &[u8], player_len: u32) {
        self.player_len = player_len;
        self.rule.update_to_default();
        self.title = title.into();
        let uuid = Uuid::new_v4();
        self.game_id = uuid.into_bytes();

        for idx in 0..self.player_len {
            let player = &mut self.players[idx as usize];
            player.score = self.rule.initial_score as i32;
        }
    }

    pub fn shuffle(&mut self) {
        self.taku = TakuT::create_shuffled()
    }

    pub fn load(&mut self, hai_ids: &[u32]) {
        self.taku = TakuT::load(hai_ids);
    }

    pub fn next_cursol(&mut self) {
        if self.is_non_duplicate {
            self.taku_cursol += 1;
        } else {
            self.players[self.teban as usize].cursol += 1;
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

    pub fn remain(&self) -> u32 {
        if self.is_non_duplicate {
            136 - self.taku_cursol
        } else {
            let start_of_yama = [14, 45, 75, 105];
            136 - 14
                - self.players[0..self.player_len as usize]
                    .iter()
                    .enumerate()
                    .map(|(idx, x)| x.cursol - start_of_yama[idx])
                    .sum::<u32>()
        }
    }

    pub fn start(&mut self, play_log: &mut PlayLog) {
        // 配牌
        self.taku_cursol = 14;
        self.dora_len = 1;
        self.uradora_len = 0;
        self.seq = 0;
        let dt = Utc::now();
        self.kyoku_id = (dt.timestamp() / (24 * 3600) * 100000) as u64;
        let mut kazes = [Some(0), Some(0), Some(0), Some(0)];

        for idx in 0..self.player_len {
            kazes[idx as usize] = Some(self.get_zikaze(idx as usize) as i32);
        }

        let uuid = Uuid::from_bytes_ref(&self.game_id);

        play_log.append_kyoku_log(
            self.kyoku_id,
            uuid.hyphenated().to_string(),
            0,
            self.tsumobou as i32,
            self.riichibou as i32,
            &self
                .players
                .iter()
                .map(|p| Some(p.score))
                .collect::<Vec<Option<i32>>>(),
            &kazes,
        );

        for idx in 0..self.player_len {
            let player = &mut self.players[idx as usize];
            let cursol: &mut u32;

            player.cursol = 14 + (idx * if idx < 2 { 31 } else { 30 });

            if self.is_non_duplicate {
                cursol = &mut self.taku_cursol;
            } else {
                cursol = &mut player.cursol;
            }
            let r = self
                .taku
                .get_range((*cursol as usize)..(*cursol + 13) as usize);

            if let Ok(mut v) = r {
                v.sort_unstable();
                for (i, item) in v.into_iter().enumerate() {
                    player.tehai[i] = item;
                }
                player.tehai_len = 13;
            }

            play_log.append_haipais_log(
                self.kyoku_id,
                idx as i32,
                &player.tehai[..player.tehai_len as usize]
                    .into_iter()
                    .map(|x| Some(x.get_pai_id()))
                    .collect::<Vec<Option<u32>>>(),
            );

            *cursol += 13;
        }
    }

    pub fn get_player(&self, index: usize) -> PlayerT {
        self.players[index].clone()
    }

    pub fn tsumo(&mut self, play_log: &mut PlayLog) -> anyhow::Result<()> {
        let player = &mut self.players[self.teban as usize];
        player.is_tsumo = true;

        if self.is_non_duplicate {
            player.tsumohai = self.taku.get(self.taku_cursol as usize)?;
        } else {
            player.tsumohai = self.taku.get(player.cursol as usize)?;
        }

        play_log.append_actions_log(
            self.kyoku_id,
            self.teban as i32,
            self.seq as i32,
            String::from("tsumo"),
            player.tsumohai.get_pai_id(),
        );
        self.seq += 1;

        self.next_cursol();

        Ok(())
    }

    pub fn sutehai(&mut self, play_log: &mut PlayLog, index: usize, is_riichi: bool) {
        let player = &mut self.players[self.teban as usize];
        let mut tehai: Vec<PaiT> = player.tehai.iter().cloned().collect();

        if is_riichi {
            player.is_riichi = true;
            player.is_ippatsu = true;
            player.score -= 1000;
        } else {
            player.is_ippatsu = false;
        }

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
        play_log.append_actions_log(
            self.kyoku_id,
            self.teban as i32,
            self.seq as i32,
            String::from("sutehai"),
            player.kawahai[player.kawahai_len as usize].get_pai_id(),
        );
        self.seq += 1;

        player.kawahai_len += 1;
        player.tsumohai = Default::default();

        player.is_tsumo = false;
        self.teban += 1;
        if self.teban == self.player_len {
            self.teban = 0;
        }
    }

    pub fn tsumo_agari(&mut self, play_log: &mut PlayLog) -> anyhow::Result<Agari> {
        let player = &self.players[self.teban as usize];
        let mut tehai: Vec<PaiT> = player.tehai.iter().cloned().collect();
        let machipai = player.tsumohai.clone();

        tehai.push(machipai.clone());

        let mut state = PaiState::from(&tehai);

        let all_mentsu = all_of_mentsu(&mut state, 0);
        let all_mentsu_w_machi = add_machi_to_mentsu(&all_mentsu, &player.tsumohai.pack());

        ensure!(all_mentsu_w_machi.len() > 0, "チョンボ！");

        let best_agari =
            self.get_best_agari(self.teban as usize, &all_mentsu_w_machi, &Vec::new(), 0)?;
        self.players[self.teban as usize].score += best_agari.score;

        let dora_orig = self
            .get_dora()
            .iter()
            .map(|x| Some(x.get_pai_id()))
            .collect_vec();
        let uradora_orig = self
            .get_uradora()
            .iter()
            .map(|x| Some(x.get_pai_id()))
            .collect_vec();

        play_log.append_agaris_log(
            self.kyoku_id,
            machipai.get_pai_id(),
            best_agari.score,
            best_agari.fu,
            best_agari.han,
            &tehai.iter().map(|x| Some(x.get_pai_id())).collect_vec(),
            &best_agari.yaku,
            &dora_orig,
            &uradora_orig,
            &dora_orig,
            &uradora_orig,
            self.teban as i32,
            self.teban as i32,
            &[Some(best_agari.score), Some(0), Some(0), Some(0)],
            false,
            0,
        );

        Ok(best_agari)
    }

    pub fn nagare(&mut self, play_log: &mut PlayLog) {
        let score = [Some(-3000), Some(0), Some(0), Some(0)];
        play_log.append_nagare_log(self.kyoku_id, String::from("流局"), &score);
    }

    pub fn action(
        &mut self,
        play_log: &mut PlayLog,
        action_type: ActionType,
        player_index: usize,
        param: u32,
    ) -> anyhow::Result<()> {
        match action_type {
            ActionType::ACTION_RIICHI => {
                if player_index == self.teban as usize {
                    self.sutehai(play_log, param as usize, true);
                    Ok(())
                } else {
                    bail!("not teban")
                }
            }
            ActionType::ACTION_SYNC => {
                if player_index == self.teban as usize {
                    self.tsumo(play_log)
                } else {
                    Ok(())
                }
            }
            ActionType::ACTION_SUTEHAI => {
                if player_index == self.teban as usize {
                    self.sutehai(play_log, param as usize, false);
                    Ok(())
                } else {
                    bail!("not teban")
                }
            }
            ActionType::ACTION_CHII => todo!(),
            ActionType::ACTION_PON => todo!(),
            ActionType::ACTION_KAN => todo!(),
            ActionType::ACTION_TSUMO => {
                if player_index == self.teban as usize {
                    self.tsumo_agari(play_log)?;
                    Ok(())
                } else {
                    bail!("not teban")
                }
            }
            ActionType::ACTION_NAGASHI => todo!(),
            _ => todo!(),
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
