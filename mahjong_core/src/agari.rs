use anyhow::bail;

use crate::mahjong_generated::open_mahjong::{
    GameStateT, Mentsu, MentsuFlag, MentsuType, Pai, PlayerT,
};

#[derive(Default, Debug)]
pub struct Shuntsu {
    m: [i32; 9],
    p: [i32; 9],
    s: [i32; 9],
}

#[derive(Default, Debug)]
pub struct Koutsu {
    m: [i32; 9],
    p: [i32; 9],
    s: [i32; 9],
    z: [i32; 7],
}

#[derive(Default, Debug)]
pub struct AgariState {
    pub fu: i32,
    pub menzen: bool,
    pub tsumo: bool,
    pub shuntsu: Shuntsu,
    pub koutsu: Koutsu,
    pub toitsu: Koutsu,
    pub n_toitsu: i32,
    pub n_shuntsu: i32,
    pub n_koutsu: i32,
    pub n_ankou: i32,
    pub n_kantsu: i32,
    pub n_yaochu: i32,
    pub n_zihai: i32,
    pub tanki: bool,
    pub pinfu: bool,
    pub kokushi: bool,
    pub churen: bool,
    pub bakaze: u32,
    pub zikaze: u32,
}

#[derive(Default, Debug)]
pub struct Agari {
    pub score: i32,
    pub fu: i32,
    pub han: i32,
    pub yaku: Vec<(String, i32)>, // 役名, 飜数
}

fn is_tanki(mentsu: &Mentsu) -> bool {
    mentsu
        .pai_list()
        .iter()
        .any(|x| x.flag() == MentsuFlag::FLAG_AGARI)
}

fn is_kanchan(mentsu: &Mentsu) -> bool {
    let index = mentsu
        .pai_list()
        .iter()
        .enumerate()
        .find(|(_, x)| x.flag() == MentsuFlag::FLAG_AGARI);

    if let Some((idx, _)) = index {
        if idx == 1 {
            return true;
        }
    }

    false
}

fn is_penchan(mentsu: &Mentsu) -> bool {
    let index = mentsu
        .pai_list()
        .iter()
        .enumerate()
        .find(|(_, x)| x.flag() == MentsuFlag::FLAG_AGARI);

    if let Some((idx, pai)) = index {
        if (idx == 2 && (pai.pai_num() % 9) == 2) || (idx == 0 && (pai.pai_num() % 9) == 6) {
            return true;
        }
    }

    false
}

pub trait AgariBehavior {
    fn get_agari(&self, who: usize, mentsu: &Vec<Mentsu>, fulo: &Vec<Mentsu>) -> AgariState;
    fn get_condition_yaku(&self, who: usize, state: &AgariState) -> Vec<(String, i32)>;
    fn get_dora_yaku(
        &self,
        who: usize,
        mentsu: &Vec<Mentsu>,
        fulo: &Vec<Mentsu>,
        nukidora: usize,
    ) -> Vec<(String, i32)>;
    fn get_best_agari(
        &self,
        who: usize,
        mentsu: &Vec<Vec<Mentsu>>,
        fulo: &Vec<Mentsu>,
        nukidora: usize,
    ) -> anyhow::Result<Agari>;
}

pub fn add_machi_to_mentsu(mentsu: &Vec<Vec<Mentsu>>, p: &Pai) -> Vec<Vec<Mentsu>> {
    let mut result = Vec::new();

    // 各mentsu_vecに対して処理を行う
    for mentsu_vec in mentsu.iter() {
        let mut positions = Vec::new();

        // ツモフラグを立てるべき牌の位置を探す
        for (index, mentsu_subvec) in mentsu_vec.iter().enumerate() {
            for (index2, mentsu_sub) in mentsu_subvec.pai_list().iter().enumerate() {
                if mentsu_sub.pai_num() == p.pai_num() {
                    positions.push((index, index2));
                    break;
                }
            }
        }

        if !positions.is_empty() {
            // ツモのすべての組み合わせを生成
            for &pos in &positions {
                let temp_vecs = mentsu_vec
                    .iter()
                    .enumerate()
                    .map(|(idx, mentsu)| {
                        if idx == pos.0 {
                            let mut mentsu_t = mentsu.unpack();
                            let pai = &mut mentsu_t.pai_list[pos.1];
                            pai.flag = MentsuFlag::FLAG_AGARI;

                            mentsu_t.pack()
                        } else {
                            mentsu.clone()
                        }
                    })
                    .collect::<Vec<Mentsu>>();
                result.push(temp_vecs);
            }
        }
    }

    result
}

fn dora_pai_num(pai_num: u8) -> u8 {
    // 北
    if pai_num == 30 {
        return 27;
    }

    // 中
    if pai_num == 33 {
        return 31;
    }

    if pai_num % 9 == 8 {
        return pai_num - 8;
    }

    pai_num + 1
}

// 状態役
fn is_riichi(_state: &AgariState, player: &PlayerT) -> Option<(String, i32)> {
    if player.is_riichi {
        Some(("立直".to_string(), 1))
    } else {
        None
    }
}

fn is_ippatsu(_state: &AgariState, player: &PlayerT) -> Option<(String, i32)> {
    if player.is_ippatsu {
        Some(("一発".to_string(), 1))
    } else {
        None
    }
}

impl AgariBehavior for GameStateT {
    fn get_agari(&self, who: usize, mentsu: &Vec<Mentsu>, fulo: &Vec<Mentsu>) -> AgariState {
        let mut agari = AgariState {
            fu: 20,
            menzen: true,
            tsumo: true,
            shuntsu: Default::default(),
            koutsu: Default::default(),
            toitsu: Default::default(),
            n_toitsu: 0,
            n_shuntsu: 0,
            n_koutsu: 0,
            n_ankou: 0,
            n_kantsu: 0,
            n_yaochu: 0,
            n_zihai: 0,
            kokushi: false,
            churen: false,
            tanki: false,
            pinfu: false,
            bakaze: self.bakaze,
            zikaze: self.get_zikaze(who),
        };

        if fulo.len() > 0 {
            agari.menzen = false;
        }

        for item in mentsu {
            match item.mentsu_type() {
                MentsuType::TYPE_ATAMA => {
                    let num = item.pai_list().get(0).pai_num();
                    agari.n_toitsu += 1;
                    if is_tanki(item) {
                        agari.tanki = true;
                        agari.fu += 2;
                    }
                    if num >= 27 {
                        if (num % 7) == self.bakaze as u8 {
                            agari.fu += 2;
                        }
                        if (num % 7) == agari.zikaze as u8 {
                            agari.fu += 2;
                        }
                        if num >= 31 {
                            agari.fu += 2;
                        }

                        agari.n_zihai += 1;
                        agari.n_yaochu += 1;
                        agari.toitsu.z[(num - 27) as usize] += 1;
                    } else if num >= 18 {
                        agari.toitsu.s[(num % 9) as usize] += 1;
                    } else if num >= 9 {
                        agari.toitsu.p[(num % 9) as usize] += 1;
                    } else {
                        agari.toitsu.m[(num % 9) as usize] += 1;
                    }

                    if (num % 9) == 0 || (num % 9) == 8 {
                        agari.n_yaochu += 1;
                    }
                }
                MentsuType::TYPE_KOUTSU => {
                    agari.n_koutsu += 1;
                    agari.n_ankou += 1;
                    let mut fu = 4;
                    let num = item.pai_list().get(0).pai_num();

                    if num >= 27 {
                        // 字牌
                        fu *= 2;
                        agari.n_zihai += 1;
                        agari.n_yaochu += 1;
                        agari.koutsu.z[(num - 27) as usize] += 1;
                    } else if num >= 18 {
                        // 索子
                        if num == 18 || num == 26 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }
                        agari.koutsu.s[(num % 9) as usize] += 1;
                    } else if num >= 9 {
                        // 筒子
                        if num == 9 || num == 17 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }
                        agari.koutsu.p[(num % 9) as usize] += 1;
                    } else {
                        // 萬子
                        if num == 0 || num == 8 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }

                        agari.koutsu.m[(num % 9) as usize] += 1;
                    }

                    agari.fu += fu;
                }
                MentsuType::TYPE_SHUNTSU => {
                    agari.n_shuntsu += 1;
                    if is_kanchan(item) {
                        agari.fu += 2;
                    }

                    if is_penchan(item) {
                        agari.fu += 2;
                    }

                    let num = item.pai_list().get(0).pai_num();

                    if num >= 18 {
                        // 索子
                        agari.shuntsu.s[(num % 9) as usize] += 1;
                        if num == 18 || num == 24 {
                            agari.n_yaochu += 1;
                        }
                    } else if num >= 9 {
                        // 筒子
                        agari.shuntsu.p[(num % 9) as usize] += 1;
                        if num == 9 || num == 15 {
                            agari.n_yaochu += 1;
                        }
                    } else {
                        // 萬子
                        agari.shuntsu.m[(num % 9) as usize] += 1;

                        if num == 0 || num == 6 {
                            agari.n_yaochu += 1;
                        }
                    }
                }
                _ => {}
            }
        }

        for item in fulo {
            match item.mentsu_type() {
                MentsuType::TYPE_ANKAN => {
                    agari.n_kantsu += 1;
                    agari.n_ankou += 1;
                    let mut fu = 16;
                    let num = item.pai_list().get(0).pai_num();
                    if num >= 27 {
                        // 字牌
                        fu *= 2;
                        agari.n_zihai += 1;
                        agari.n_yaochu += 1;
                        agari.koutsu.z[(num - 27) as usize] += 1;
                    } else if num >= 18 {
                        // 索子
                        if num == 18 || num == 26 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }
                        agari.koutsu.s[(num % 9) as usize] += 1;
                    } else if num >= 9 {
                        // 筒子
                        if num == 9 || num == 17 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }
                        agari.koutsu.p[(num % 9) as usize] += 1;
                    } else {
                        // 萬子
                        if num == 0 || num == 8 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }

                        agari.koutsu.m[(num % 9) as usize] += 1;
                    }

                    agari.fu += fu;
                }
                MentsuType::TYPE_MINKAN => {
                    agari.n_kantsu += 1;
                    let mut fu = 8;
                    let num = item.pai_list().get(0).pai_num();
                    if num >= 27 {
                        // 字牌
                        fu *= 2;
                        agari.n_zihai += 1;
                        agari.n_yaochu += 1;
                        agari.koutsu.z[(num - 27) as usize] += 1;
                    } else if num >= 18 {
                        // 索子
                        if num == 18 || num == 26 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }
                        agari.koutsu.s[(num % 9) as usize] += 1;
                    } else if num >= 9 {
                        // 筒子
                        if num == 9 || num == 17 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }
                        agari.koutsu.p[(num % 9) as usize] += 1;
                    } else {
                        // 萬子
                        if num == 0 || num == 8 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }

                        agari.koutsu.m[(num % 9) as usize] += 1;
                    }

                    agari.fu += fu;
                }
                MentsuType::TYPE_KOUTSU => {
                    agari.n_koutsu += 1;
                    let mut fu = 2;
                    let num = item.pai_list().get(0).pai_num();

                    if num >= 27 {
                        // 字牌
                        fu *= 2;
                        agari.n_zihai += 1;
                        agari.n_yaochu += 1;
                        agari.koutsu.z[(num - 27) as usize] += 1;
                    } else if num >= 18 {
                        // 索子
                        if num == 18 || num == 26 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }
                        agari.koutsu.s[(num % 9) as usize] += 1;
                    } else if num >= 9 {
                        // 筒子
                        if num == 9 || num == 17 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }
                        agari.koutsu.p[(num % 9) as usize] += 1;
                    } else {
                        // 萬子
                        if num == 0 || num == 8 {
                            fu *= 2;
                            agari.n_yaochu += 1;
                        }

                        agari.koutsu.m[(num % 9) as usize] += 1;
                    }

                    agari.fu += fu;
                }
                MentsuType::TYPE_SHUNTSU => {
                    agari.n_shuntsu += 1;
                    let num = item.pai_list().get(0).pai_num();

                    if num >= 18 {
                        // 索子
                        agari.shuntsu.s[(num % 9) as usize] += 1;
                        if num == 18 || num == 24 {
                            agari.n_yaochu += 1;
                        }
                    } else if num >= 9 {
                        // 筒子
                        agari.shuntsu.p[(num % 9) as usize] += 1;
                        if num == 9 || num == 15 {
                            agari.n_yaochu += 1;
                        }
                    } else {
                        // 萬子
                        agari.shuntsu.m[(num % 9) as usize] += 1;

                        if num == 0 || num == 6 {
                            agari.n_yaochu += 1;
                        }
                    }
                }
                _ => {}
            }
        }

        if agari.n_toitsu >= 7 {
            // チートイツ
            agari.fu = 25;
        } else {
            agari.pinfu = agari.menzen && agari.fu == 20;

            if agari.tsumo {
                if !agari.pinfu {
                    agari.fu += 2;
                }
            } else {
                if agari.menzen {
                    agari.fu += 10;
                } else if agari.fu == 20 {
                    agari.fu = 30;
                }
            }

            agari.fu = (agari.fu + 9) / 10 * 10;
        }

        agari
    }

    fn get_condition_yaku(&self, who: usize, state: &AgariState) -> Vec<(String, i32)> {
        let check_list = [is_riichi, is_ippatsu];

        check_list
            .iter()
            .flat_map(|f| f(state, &self.players[who]))
            .collect()
    }

    fn get_dora_yaku(
        &self,
        who: usize,
        mentsu: &Vec<Mentsu>,
        fulo: &Vec<Mentsu>,
        nukidora: usize,
    ) -> Vec<(String, i32)> {
        let mut ret = Vec::new();
        let dora_pais = self.get_dora();
        let uradora_pais = self.get_uradora();
        let player = &self.players[who];

        let dora_num = mentsu
            .iter()
            .chain(fulo.iter())
            .flat_map(|m| {
                m.pai_list().iter().take(m.pai_len() as usize).map(|p| {
                    dora_pais
                        .iter()
                        .filter(|d| dora_pai_num(d.pai_num) == p.pai_num())
                        .count()
                })
            })
            .reduce(|acc, e| acc + e)
            .unwrap_or(0)
            + nukidora;
        let uradora_num = match player.is_riichi {
            true => mentsu
                .iter()
                .chain(fulo.iter())
                .flat_map(|m| {
                    m.pai_list().iter().take(m.pai_len() as usize).map(|p| {
                        uradora_pais
                            .iter()
                            .filter(|d| dora_pai_num(d.pai_num) == p.pai_num())
                            .count()
                    })
                })
                .reduce(|acc, e| acc + e)
                .unwrap_or(0),
            false => 0,
        };

        if dora_num > 0 {
            ret.push(("ドラ".to_string(), dora_num as i32));
        }

        if uradora_num > 0 {
            ret.push(("裏ドラ".to_string(), uradora_num as i32));
        }
        ret
    }

    fn get_best_agari(
        &self,
        who: usize,
        mentsu: &Vec<Vec<Mentsu>>,
        fulo: &Vec<Mentsu>,
        nukidora: usize,
    ) -> anyhow::Result<Agari> {
        let ret = mentsu
            .iter()
            .map(|m| {
                let agari = self.get_agari(who, m, fulo);
                let mut yakus = self.get_condition_yaku(who, &agari);
                yakus.extend(agari.get_yaku_list());
                yakus.extend(self.get_dora_yaku(who, m, fulo, nukidora));
                agari.get_agari(&yakus)
            })
            .max_by_key(|x| x.score);

        if let Some(x) = ret {
            Ok(x)
        } else {
            bail!("Agari tehai or yaku not found")
        }
    }
}

const KAZE_STR: [char; 4] = ['東', '南', '西', '北'];

fn is_menzen_tsumo(state: &AgariState) -> Option<(String, i32)> {
    if state.menzen && state.tsumo {
        Some(("門前清自摸和".to_string(), 1))
    } else {
        None
    }
}

fn is_tanyao(state: &AgariState) -> Option<(String, i32)> {
    if state.n_yaochu == 0 {
        Some(("断么九".to_string(), 1))
    } else {
        None
    }
}

fn is_pinfu(state: &AgariState) -> Option<(String, i32)> {
    if state.pinfu {
        Some(("平和".to_string(), 1))
    } else {
        None
    }
}

fn is_bakaze(state: &AgariState) -> Option<(String, i32)> {
    if state.koutsu.z[state.bakaze as usize] == 1 {
        Some((
            format!("場風 {}", KAZE_STR[state.bakaze as usize]).to_string(),
            1,
        ))
    } else {
        None
    }
}

fn is_zikaze(state: &AgariState) -> Option<(String, i32)> {
    if state.koutsu.z[state.zikaze as usize] == 1 {
        Some((
            format!("自風 {}", KAZE_STR[state.zikaze as usize]).to_string(),
            1,
        ))
    } else {
        None
    }
}

fn is_haku(state: &AgariState) -> Option<(String, i32)> {
    if state.koutsu.z[4] == 1 {
        Some(("役牌 白".to_string(), 1))
    } else {
        None
    }
}

fn is_hatsu(state: &AgariState) -> Option<(String, i32)> {
    if state.koutsu.z[5] == 1 {
        Some(("役牌 發".to_string(), 1))
    } else {
        None
    }
}

fn is_chun(state: &AgariState) -> Option<(String, i32)> {
    if state.koutsu.z[6] == 1 {
        Some(("役牌 中".to_string(), 1))
    } else {
        None
    }
}

fn is_iipeikou(state: &AgariState) -> Option<(String, i32)> {
    if !state.menzen {
        return None;
    }

    let peikou = state
        .shuntsu
        .m
        .iter()
        .chain(state.shuntsu.p.iter())
        .chain(state.shuntsu.s.iter())
        .map(|x| x >> 1)
        .reduce(|acc, e| acc + e);
    if let Some(x) = peikou {
        if x == 1 {
            return Some(("一盃口".to_string(), 1));
        }
    }

    None
}

fn is_sanshoku_doushun(state: &AgariState) -> Option<(String, i32)> {
    for i in 0..7 {
        if state.shuntsu.m[i] > 0 && state.shuntsu.p[i] > 0 && state.shuntsu.s[i] > 0 {
            return Some(("三色同順".to_string(), if state.menzen { 2 } else { 1 }));
        }
    }

    None
}

fn is_ittsu(state: &AgariState) -> Option<(String, i32)> {
    if state.shuntsu.m[0] > 0 && state.shuntsu.m[3] > 0 && state.shuntsu.m[6] > 0 {
        return Some(("一気通貫".to_string(), if state.menzen { 2 } else { 1 }));
    }
    if state.shuntsu.p[0] > 0 && state.shuntsu.p[3] > 0 && state.shuntsu.p[6] > 0 {
        return Some(("一気通貫".to_string(), if state.menzen { 2 } else { 1 }));
    }
    if state.shuntsu.s[0] > 0 && state.shuntsu.s[3] > 0 && state.shuntsu.s[6] > 0 {
        return Some(("一気通貫".to_string(), if state.menzen { 2 } else { 1 }));
    }

    None
}

fn is_chanta(state: &AgariState) -> Option<(String, i32)> {
    if state.n_yaochu >= 5 && state.n_zihai > 0 && state.n_shuntsu > 0 {
        return Some(("混全帯么九".to_string(), if state.menzen { 2 } else { 1 }));
    }

    None
}

fn is_chitoi(state: &AgariState) -> Option<(String, i32)> {
    if state.n_toitsu >= 7 {
        return Some(("七対子".to_string(), 2));
    }

    None
}

fn is_toitoi(state: &AgariState) -> Option<(String, i32)> {
    if state.n_koutsu >= 4 {
        return Some(("対々和".to_string(), 2));
    }

    None
}

fn is_sanankou(state: &AgariState) -> Option<(String, i32)> {
    if state.n_ankou == 3 {
        return Some(("三暗刻".to_string(), 2));
    }

    None
}

fn is_sankantsu(state: &AgariState) -> Option<(String, i32)> {
    if state.n_kantsu == 3 {
        return Some(("三槓子".to_string(), 2));
    }

    None
}

fn is_sanshoku_doukou(state: &AgariState) -> Option<(String, i32)> {
    for i in 0..9 {
        if state.koutsu.m[i] > 0 && state.koutsu.p[i] > 0 && state.koutsu.s[i] > 0 {
            return Some(("三色同刻".to_string(), 2));
        }
    }

    None
}

fn is_honroutou(state: &AgariState) -> Option<(String, i32)> {
    if state.n_shuntsu == 0 && state.n_zihai > 0 {
        if (state.n_yaochu >= 4 && state.n_koutsu == 4)
            || (state.n_yaochu >= 7 && state.n_toitsu >= 7)
        {
            return Some(("混老頭".to_string(), 2));
        }
    }

    None
}

fn is_shosangen(state: &AgariState) -> Option<(String, i32)> {
    if state.koutsu.z[4] + state.koutsu.z[5] + state.koutsu.z[6] == 2
        && state.toitsu.z[4] + state.toitsu.z[5] + state.toitsu.z[6] == 1
    {
        return Some(("小三元".to_string(), 2));
    }

    None
}

fn is_honitsu(state: &AgariState) -> Option<(String, i32)> {
    if state.n_zihai == 0 {
        return None;
    }

    let manzu: i32 = state
        .shuntsu
        .m
        .iter()
        .chain(state.koutsu.m.iter())
        .chain(state.toitsu.m.iter())
        .sum();

    if manzu + state.n_zihai >= 5 {
        return Some(("混一色".to_string(), if state.menzen { 3 } else { 2 }));
    }

    let pinzu: i32 = state
        .shuntsu
        .p
        .iter()
        .chain(state.koutsu.p.iter())
        .chain(state.toitsu.p.iter())
        .sum();

    if pinzu + state.n_zihai >= 5 {
        return Some(("混一色".to_string(), if state.menzen { 3 } else { 2 }));
    }

    let souzu: i32 = state
        .shuntsu
        .s
        .iter()
        .chain(state.koutsu.s.iter())
        .chain(state.toitsu.s.iter())
        .sum();

    if souzu + state.n_zihai >= 5 {
        return Some(("混一色".to_string(), if state.menzen { 3 } else { 2 }));
    }

    None
}

fn is_junchan(state: &AgariState) -> Option<(String, i32)> {
    if state.n_yaochu >= 5 && state.n_shuntsu > 0 && state.n_zihai == 0 {
        return Some(("混全帯么九".to_string(), if state.menzen { 2 } else { 1 }));
    }

    None
}

fn is_ryampeikou(state: &AgariState) -> Option<(String, i32)> {
    if !state.menzen {
        return None;
    }

    let peikou = state
        .shuntsu
        .m
        .iter()
        .chain(state.shuntsu.p.iter())
        .chain(state.shuntsu.s.iter())
        .map(|x| x >> 1)
        .reduce(|acc, e| acc + e);
    if let Some(x) = peikou {
        if x == 2 {
            return Some(("二盃口".to_string(), 3));
        }
    }
    None
}

fn is_chinitsu(state: &AgariState) -> Option<(String, i32)> {
    if state.n_zihai > 0 {
        return None;
    }

    let manzu: i32 = state
        .shuntsu
        .m
        .iter()
        .chain(state.koutsu.m.iter())
        .chain(state.toitsu.m.iter())
        .sum();

    if manzu >= 5 {
        return Some(("清一色".to_string(), if state.menzen { 6 } else { 5 }));
    }

    let pinzu: i32 = state
        .shuntsu
        .p
        .iter()
        .chain(state.koutsu.p.iter())
        .chain(state.toitsu.p.iter())
        .sum();

    if pinzu >= 5 {
        return Some(("清一色".to_string(), if state.menzen { 6 } else { 5 }));
    }

    let souzu: i32 = state
        .shuntsu
        .s
        .iter()
        .chain(state.koutsu.s.iter())
        .chain(state.toitsu.s.iter())
        .sum();

    if souzu >= 5 {
        return Some(("清一色".to_string(), if state.menzen { 6 } else { 5 }));
    }

    None
}

fn is_kokushi(state: &AgariState) -> Option<(String, i32)> {
    if state.kokushi {
        if state.tanki {
            return Some(("国士無双１３面".to_string(), -2));
        } else {
            return Some(("国士無双".to_string(), -1));
        }
    }

    None
}

fn is_suanko(state: &AgariState) -> Option<(String, i32)> {
    if state.n_ankou >= 4 {
        if state.tanki {
            return Some(("四暗刻単騎".to_string(), -2));
        } else {
            return Some(("四暗刻".to_string(), -1));
        }
    }

    None
}

fn is_daisangen(state: &AgariState) -> Option<(String, i32)> {
    if state.koutsu.z[4] + state.koutsu.z[5] + state.koutsu.z[6] == 3 {
        return Some(("大三元".to_string(), -1));
    }

    None
}

fn is_sushiho(state: &AgariState) -> Option<(String, i32)> {
    if state.koutsu.z[0] + state.koutsu.z[1] + state.koutsu.z[2] + state.koutsu.z[3] == 4 {
        return Some(("大四喜".to_string(), -2));
    }

    if state.koutsu.z[0]
        + state.koutsu.z[1]
        + state.koutsu.z[2]
        + state.koutsu.z[3]
        + state.toitsu.z[0]
        + state.toitsu.z[1]
        + state.toitsu.z[2]
        + state.toitsu.z[3]
        >= 4
    {
        return Some(("小四喜".to_string(), -1));
    }

    None
}

fn is_tsuiso(state: &AgariState) -> Option<(String, i32)> {
    let z_toitsu: i32 = state.toitsu.z.iter().sum();
    if z_toitsu >= 7 {
        return Some(("字一色".to_string(), -1));
    }

    let zihai: i32 = state.koutsu.z.iter().chain(state.toitsu.z.iter()).sum();

    if zihai >= 5 {
        return Some(("字一色".to_string(), -1));
    }

    None
}

fn is_ryuiso(state: &AgariState) -> Option<(String, i32)> {
    let n_atama = state.toitsu.s[1]
        + state.toitsu.s[2]
        + state.toitsu.s[3]
        + state.toitsu.s[5]
        + state.toitsu.s[7]
        + state.toitsu.z[5];
    let n_shuntsu = state.shuntsu.s[1];
    let n_koutsu = state.koutsu.s[1]
        + state.koutsu.s[2]
        + state.koutsu.s[3]
        + state.koutsu.s[5]
        + state.koutsu.s[7]
        + state.koutsu.z[5];
    if n_atama > 0 && n_shuntsu + n_koutsu >= 4 {
        return Some(("緑一色".to_string(), -1));
    }

    None
}

fn is_chinroto(state: &AgariState) -> Option<(String, i32)> {
    let n_koutsu = state.koutsu.m[0]
        + state.koutsu.m[8]
        + state.koutsu.p[0]
        + state.koutsu.p[8]
        + state.koutsu.s[0]
        + state.koutsu.s[8];
    let n_atama = state.toitsu.m[0]
        + state.toitsu.m[8]
        + state.toitsu.p[0]
        + state.toitsu.p[8]
        + state.toitsu.s[0]
        + state.toitsu.s[8];
    if n_atama > 0 && n_koutsu >= 4 {
        return Some(("清老頭".to_string(), -1));
    }

    None
}

fn is_sukantsu(state: &AgariState) -> Option<(String, i32)> {
    if state.n_kantsu == 4 {
        return Some(("四槓子".to_string(), -1));
    }

    None
}

fn is_churen(state: &AgariState) -> Option<(String, i32)> {
    if state.churen {
        return Some(("九蓮宝燈".to_string(), -1));
    }

    None
}

impl AgariState {
    ///　あがり役を判定します
    pub fn get_yaku_list(&self) -> Vec<(String, i32)> {
        let check_list = [
            is_menzen_tsumo,
            is_tanyao,
            is_pinfu,
            is_bakaze,
            is_zikaze,
            is_haku,
            is_hatsu,
            is_chun,
            is_iipeikou,
            is_sanshoku_doushun,
            is_ittsu,
            is_chanta,
            is_chitoi,
            is_toitoi,
            is_sanankou,
            is_sankantsu,
            is_sanshoku_doukou,
            is_honroutou,
            is_shosangen,
            is_honitsu,
            is_junchan,
            is_ryampeikou,
            is_chinitsu,
            is_kokushi,
            is_suanko,
            is_daisangen,
            is_sushiho,
            is_tsuiso,
            is_ryuiso,
            is_chinroto,
            is_sukantsu,
            is_churen,
        ];

        check_list.iter().flat_map(|f| f(self)).collect()
    }

    /// 上がり点を計算します
    pub fn get_agari(&self, yaku: &Vec<(String, i32)>) -> Agari {
        let mut agari = Agari::default();

        if yaku.len() == 0 {
            return agari;
        }

        let yakumans: i32 = yaku.iter().filter(|x| x.1 < 0).map(|x| x.1).sum();
        let han: i32 = yaku.iter().filter(|x| x.1 > 0).map(|x| x.1).sum();

        if yakumans < 0 {
            // 役満の場合
            agari.fu = 0;
            agari.score = 32000 * -yakumans;
            agari.yaku = yaku.clone().into_iter().filter(|x| x.1 < 0).collect();
        } else {
            agari.fu = self.fu;
            agari.han = han;
            agari.yaku = yaku.clone();

            agari.score = if han >= 13 {
                32000
            } else if han >= 11 {
                24000
            } else if han >= 8 {
                16000
            } else if han >= 6 {
                12000
            } else {
                let base = (((agari.fu << (4 + han)) + 99) / 100) * 100;
                8000.min(base)
            };
        }

        agari
    }
}
