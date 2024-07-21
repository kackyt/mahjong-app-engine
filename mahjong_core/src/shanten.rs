use crate::mahjong_generated::open_mahjong::{Mentsu, MentsuFlag, MentsuPai, MentsuType, PaiT};
use itertools::iproduct;

/// 牌姿の内部表現
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PaiState {
    pub hai_count_m: [i32; 9],
    pub hai_count_p: [i32; 9],
    pub hai_count_s: [i32; 9],
    pub hai_count_z: [i32; 7],
}

pub fn shanten(mut n_mentsu: i32, mut n_tahtsu: i32, mut n_koritsu: i32, b_atama: bool) -> i32 {
    let n = if b_atama { 4 } else { 5 };
    if n_mentsu > 4 {
        n_tahtsu += n_mentsu - 4;
        n_mentsu = 4;
    }
    if n_mentsu + n_tahtsu > 4 {
        n_koritsu += n_mentsu + n_tahtsu - 4;
        n_tahtsu = 4 - n_mentsu;
    }

    if n_mentsu + n_tahtsu + n_koritsu > n {
        n_koritsu = n - n_mentsu - n_tahtsu;
    }

    if b_atama {
        n_tahtsu += 1;
    }

    return 13 - n_mentsu * 3 - n_tahtsu * 2 - n_koritsu;
}

pub fn tahtsu_koritsu_count(hai_count: &[i32; 9]) -> [(i32, i32, i32); 2] {
    let (mut n_pai, mut n_dazi, mut n_guli) = (0, 0, 0);

    for n in 0..9 {
        n_pai += hai_count[n];

        if n < 7 && hai_count[n + 1] == 0 && hai_count[n + 2] == 0 {
            n_dazi += n_pai >> 1;
            n_guli += n_pai % 2;
            n_pai = 0;
        }
    }
    n_dazi += n_pai >> 1;
    n_guli += n_pai % 2;

    [(0, n_dazi, n_guli), (0, n_dazi, n_guli)]
}

pub fn mentsu_count(hai_count: &mut [i32; 9], n: usize) -> [(i32, i32, i32); 2] {
    if n >= 9 {
        return tahtsu_koritsu_count(hai_count);
    }

    let mut max_count = mentsu_count(hai_count, n + 1);

    // 順子を抜き出す
    if n < 7 && hai_count[n] > 0 && hai_count[n + 1] > 0 && hai_count[n + 2] > 0 {
        hai_count[n] -= 1;
        hai_count[n + 1] -= 1;
        hai_count[n + 2] -= 1;
        let mut r = mentsu_count(hai_count, n);
        hai_count[n] += 1;
        hai_count[n + 1] += 1;
        hai_count[n + 2] += 1;
        r[0].0 += 1;
        r[1].0 += 1;
        if r[0].2 < max_count[0].2 || (r[0].2 == max_count[0].2 && r[0].1 < max_count[0].1) {
            max_count[0] = r[0];
        }
        if r[1].0 > max_count[1].0 || (r[1].0 == max_count[1].0 && r[1].1 > max_count[1].1) {
            max_count[1] = r[1];
        }
    }

    // 刻子を抜き出す
    if hai_count[n] >= 3 {
        hai_count[n] -= 3;
        let mut r2 = mentsu_count(hai_count, n);
        hai_count[n] += 3;
        r2[0].0 += 1;
        r2[1].0 += 1;
        if r2[0].2 < max_count[0].2 || (r2[0].2 == max_count[0].2 && r2[0].1 < max_count[0].1) {
            max_count[0] = r2[0];
        }
        if r2[1].0 > max_count[1].0 || (r2[1].0 == max_count[1].0 && r2[1].1 > max_count[1].1) {
            max_count[1] = r2[1];
        }
    }

    max_count
}

/// すべての面子を抜き出す
fn all_of_suit_mentsu(suit: usize, hai_count: &mut [i32; 9], n: usize) -> Vec<Vec<Mentsu>> {
    if n >= 9 {
        return vec![vec![]];
    }

    // メンツをすべて抜き取ったら次の位置へ進む
    if hai_count[n] == 0 {
        return all_of_suit_mentsu(suit, hai_count, n + 1);
    }

    let mut shuntsu: Vec<Vec<Mentsu>> = vec![];

    // 順子を抜き出す
    if n < 7 && hai_count[n] > 0 && hai_count[n + 1] > 0 && hai_count[n + 2] > 0 {
        let m = Mentsu::new(
            &[
                MentsuPai::new((n + suit * 9) as u8, 0, MentsuFlag::FLAG_NONE),
                MentsuPai::new((n + suit * 9 + 1) as u8, 0, MentsuFlag::FLAG_NONE),
                MentsuPai::new((n + suit * 9 + 2) as u8, 0, MentsuFlag::FLAG_NONE),
                MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
            ],
            3,
            MentsuType::TYPE_SHUNTSU,
        );
        hai_count[n] -= 1;
        hai_count[n + 1] -= 1;
        hai_count[n + 2] -= 1;
        let r = all_of_suit_mentsu(suit, hai_count, n);
        hai_count[n] += 1;
        hai_count[n + 1] += 1;
        hai_count[n + 2] += 1;

        shuntsu = r
            .into_iter()
            .map(|mut x| {
                x.insert(0, m);
                x
            })
            .collect();
    }

    let mut koutsu: Vec<Vec<Mentsu>> = vec![];

    // 刻子を抜き出す
    if hai_count[n] >= 3 {
        let m = Mentsu::new(
            &[
                MentsuPai::new((n + suit * 9) as u8, 0, MentsuFlag::FLAG_NONE),
                MentsuPai::new((n + suit * 9) as u8, 0, MentsuFlag::FLAG_NONE),
                MentsuPai::new((n + suit * 9) as u8, 0, MentsuFlag::FLAG_NONE),
                MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
            ],
            3,
            MentsuType::TYPE_KOUTSU,
        );
        hai_count[n] -= 3;
        let k = all_of_suit_mentsu(suit, hai_count, n + 1);
        hai_count[n] += 3;

        koutsu = k
            .into_iter()
            .map(|mut x| {
                x.insert(0, m);
                x
            })
            .collect();
    }

    // println!("s{} {}: {:?} {:?}\r", suit, n, shuntsu, koutsu);

    if shuntsu.len() == 0 && koutsu.len() == 0 {
        vec![vec![]]
    } else {
        [shuntsu, koutsu].concat()
    }
}

fn all_of_mentsu_without_atama(pai_state: &mut PaiState) -> Vec<Vec<Mentsu>> {
    let mut all_mentsu: Vec<Vec<Mentsu>> = vec![];

    let man = all_of_suit_mentsu(0, &mut pai_state.hai_count_m, 0);
    let pin = all_of_suit_mentsu(1, &mut pai_state.hai_count_p, 0);
    let sou = all_of_suit_mentsu(2, &mut pai_state.hai_count_s, 0);

    all_mentsu = iproduct!(man, pin, sou)
        .map(|(m, p, s)| [m, p, s].concat())
        .collect::<Vec<Vec<Mentsu>>>();

    // 字牌は刻子1通りのみ
    let mut zihai: Vec<Mentsu> = vec![];

    for n in 0..7 {
        if pai_state.hai_count_z[n] >= 3 {
            let m = Mentsu::new(
                &[
                    MentsuPai::new((n + 27) as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new((n + 27) as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new((n + 27) as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ],
                3,
                MentsuType::TYPE_KOUTSU,
            );
            zihai.push(m);
        }
    }

    all_mentsu
        .into_iter()
        .map(|x| [x, zihai.clone()].concat())
        .collect::<Vec<Vec<Mentsu>>>()
}

pub fn all_of_mentsu(pai_state: &mut PaiState, n_fulo: usize) -> Vec<Vec<Mentsu>> {
    let mut all_mentsu: Vec<Vec<Mentsu>> = vec![];
    for n in 0..9 {
        if pai_state.hai_count_m[n] >= 2 {
            let m = Mentsu::new(
                &[
                    MentsuPai::new(n as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(n as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ],
                2,
                MentsuType::TYPE_ATAMA,
            );
            pai_state.hai_count_m[n] -= 2;
            let r = all_of_mentsu_without_atama(pai_state);
            pai_state.hai_count_m[n] += 2;
            all_mentsu.extend(
                r.into_iter()
                    .map(|mut x| {
                        x.insert(0, m);
                        x
                    })
                    .collect::<Vec<Vec<Mentsu>>>(),
            );
        }

        if pai_state.hai_count_p[n] >= 2 {
            let m = Mentsu::new(
                &[
                    MentsuPai::new((n + 9) as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new((n + 9) as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ],
                2,
                MentsuType::TYPE_ATAMA,
            );
            pai_state.hai_count_p[n] -= 2;
            let r = all_of_mentsu_without_atama(pai_state);
            pai_state.hai_count_p[n] += 2;
            all_mentsu.extend(
                r.into_iter()
                    .map(|mut x| {
                        x.insert(0, m);
                        x
                    })
                    .collect::<Vec<Vec<Mentsu>>>(),
            );
        }

        if pai_state.hai_count_s[n] >= 2 {
            let m = Mentsu::new(
                &[
                    MentsuPai::new((n + 18) as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new((n + 18) as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ],
                2,
                MentsuType::TYPE_ATAMA,
            );
            pai_state.hai_count_s[n] -= 2;
            let r = all_of_mentsu_without_atama(pai_state);
            pai_state.hai_count_s[n] += 2;
            all_mentsu.extend(
                r.into_iter()
                    .map(|mut x| {
                        x.insert(0, m);
                        x
                    })
                    .collect::<Vec<Vec<Mentsu>>>(),
            );
        }

        if n < 7 && pai_state.hai_count_z[n] >= 2 {
            let m = Mentsu::new(
                &[
                    MentsuPai::new((n + 27) as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new((n + 27) as u8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ],
                2,
                MentsuType::TYPE_ATAMA,
            );
            pai_state.hai_count_z[n] -= 2;
            let r = all_of_mentsu_without_atama(pai_state);
            pai_state.hai_count_z[n] += 2;
            all_mentsu.extend(
                r.into_iter()
                    .map(|mut x| {
                        x.insert(0, m);
                        x
                    })
                    .collect::<Vec<Vec<Mentsu>>>(),
            );
        }
    }
    all_mentsu
        .into_iter()
        .filter(|x| x.len() + n_fulo >= 5)
        .collect::<Vec<Vec<Mentsu>>>()
}

impl PaiState {
    pub fn from(value: &[PaiT]) -> Self {
        let mut state = PaiState::default();
        state.init(value);
        state
    }

    pub fn init(&mut self, value: &[PaiT]) {
        for n in 0..9 {
            self.hai_count_m[n] = 0;
            self.hai_count_p[n] = 0;
            self.hai_count_s[n] = 0;
        }
        for n in 0..7 {
            self.hai_count_z[n] = 0;
        }

        for hai in value.iter() {
            let num = hai.pai_num as usize;
            if num < 9 {
                self.hai_count_m[num] += 1;
            } else if num < 18 {
                self.hai_count_p[num - 9] += 1;
            } else if num < 27 {
                self.hai_count_s[num - 18] += 1;
            } else {
                self.hai_count_z[num - 27] += 1;
            }
        }
    }

    pub fn append(&mut self, hai: &PaiT) {
        let num = hai.pai_num as usize;
        if num < 9 {
            self.hai_count_m[num] += 1;
        } else if num < 18 {
            self.hai_count_p[num - 9] += 1;
        } else if num < 27 {
            self.hai_count_s[num - 18] += 1;
        } else {
            self.hai_count_z[num - 27] += 1;
        }
    }

    fn get_shanten_case(&mut self, b_atama: bool, n_fulo: usize) -> i32 {
        let m = mentsu_count(&mut self.hai_count_m, 0);
        let p = mentsu_count(&mut self.hai_count_p, 0);
        let s = mentsu_count(&mut self.hai_count_s, 0);
        let mut z = (0, 0, 0);

        for n in 0..7 {
            if self.hai_count_z[n] >= 3 {
                z.0 += 1;
            } else if self.hai_count_z[n] == 2 {
                z.1 += 1;
            } else if self.hai_count_z[n] == 1 {
                z.2 += 1;
            }
        }

        let mut min_shanten = 13;

        for (manzu, pinzu, souzu) in iproduct!(&m, &p, &s) {
            let x = (
                (n_fulo as i32) + manzu.0 + pinzu.0 + souzu.0 + z.0,
                manzu.1 + pinzu.1 + souzu.1 + z.1,
                manzu.2 + pinzu.2 + souzu.2 + z.2,
            );

            min_shanten = min_shanten.min(shanten(x.0, x.1, x.2, b_atama));
        }

        min_shanten
    }

    pub fn get_shanten(&mut self, n_fulo: usize) -> i32 {
        let mut min_shanten = self.get_shanten_case(false, n_fulo);

        // 可能な雀頭を抜き取り、雀頭ありの場合のシャンテン数を計算する
        for n in 0..9 {
            if self.hai_count_m[n] >= 2 {
                self.hai_count_m[n] -= 2;
                min_shanten = min_shanten.min(self.get_shanten_case(true, n_fulo));
                self.hai_count_m[n] += 2;
            }
            if self.hai_count_p[n] >= 2 {
                self.hai_count_p[n] -= 2;
                min_shanten = min_shanten.min(self.get_shanten_case(true, n_fulo));
                self.hai_count_p[n] += 2;
            }
            if self.hai_count_s[n] >= 2 {
                self.hai_count_s[n] -= 2;
                min_shanten = min_shanten.min(self.get_shanten_case(true, n_fulo));
                self.hai_count_s[n] += 2;
            }
        }

        for n in 0..7 {
            if self.hai_count_z[n] >= 2 {
                self.hai_count_z[n] -= 2;
                min_shanten = min_shanten.min(self.get_shanten_case(true, n_fulo));
                self.hai_count_z[n] += 2;
            }
        }

        min_shanten
    }
}

impl PaiT {
    pub fn is_valid(&self) -> bool {
        let num = self.pai_num;
        let id = self.id;

        if id >= 4 {
            return false;
        }

        if num >= 33 {
            return false;
        }

        true
    }
}
