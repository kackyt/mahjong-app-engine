use crate::{mahjong_generated::open_mahjong::{PaiT, MentsuT, Mentsu, Pai, MentsuPai, MentsuFlag}, shanten::PaiState};


#[derive(Default)]
struct Shuntsu {
    m: [i32; 9],
    p: [i32; 9],
    s: [i32; 9],
}

#[derive(Default)]
struct Koutsu {
    m: [i32; 9],
    p: [i32; 9],
    s: [i32; 9],
    z: [i32; 7],
}

#[derive(Default)]
struct AgariState {
    fu: i32,
    menzen: bool,
    tsumo: bool,
    shuntsu: Shuntsu,
    koutsu: Koutsu,
    n_shuntsu: i32,
    n_koutsu: i32,
    n_ankou: i32,
    n_kantsu: i32,
    n_yaochu: i32,
    n_zihai: i32,
    tanki: bool,
    pinfu: bool,
    bakaze: i32,
    zikaze: i32
}


#[derive(Default)]
struct Agari {
    machihai: String,
    ten: i32,
    fu: i32,
    han: i32,
    yaku: Vec<(String, i32)>, // 役名, 飜数
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
                let temp_vecs = mentsu_vec.iter().enumerate().map(|(idx, mentsu)| {
                    if idx == pos.0 {
                        let mut mentsu_t = mentsu.unpack();
                        let pai = &mut mentsu_t.pai_list[pos.1];
                        pai.flag = MentsuFlag::FLAG_TSUMO;

                        mentsu_t.pack()
                    } else {
                        mentsu.clone()
                    }
                }).collect::<Vec<Mentsu>>();
                result.push(temp_vecs);
            }
        }
    }

    result
}



#[cfg(test)]
mod tests {
    use crate::mahjong_generated::open_mahjong::MentsuType;

    use super::*;

    #[test]
    fn test_add_machi_to_mentsu() {
        let mentsu = vec![
            vec![
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(5, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(6, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(7, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(9, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
        ];

        let p = Pai::new(2, 0, false, false, false);

        let result = add_machi_to_mentsu(&mentsu, &p);
        
        assert_eq!(result, vec![
            vec![
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_TSUMO),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(5, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(6, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(7, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(9, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_TSUMO),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
        ]);
    }


    #[test]
    fn test_add_machi_to_mentsu2() {
        let mentsu = vec![
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_KOUTSU),
                Mentsu::new(&[
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(5, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(6, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
        ];

        let p = Pai::new(2, 0, false, false, false);

        let result = add_machi_to_mentsu(&mentsu, &p);
        
        assert_eq!(result, vec![
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_TSUMO),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_KOUTSU),
                Mentsu::new(&[
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(5, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(6, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_TSUMO),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_TSUMO),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
        ]);
    }

}


/// 一般形の上がりメンツを作ります
fn make_agari_ippankei(tehai: &Vec<PaiT>, fulo: &Vec<MentsuT>) -> Vec<MentsuT> {
    let mut paistate: PaiState = PaiState::from(tehai);
    let mut agari: Vec<MentsuT> = Vec::new();

    agari
}

///　あがり役を判定します
fn get_yaku(tehai: &Vec<MentsuT>, machihai: &PaiT, fulo: &Vec<MentsuT>, num_dora: i32) -> Vec<(String, i32)> {
    let mut yaku: Vec<(String, i32)> = Vec::new();

    /* 
    // 七対子
    if is_chitoi(tehai) {
        yaku.push(("七対子".to_string(), 2));
    }

    // 国士無双
    if is_kokushi(tehai) {
        yaku.push(("国士無双".to_string(), 13));
    }

    // 純正九蓮宝燈
    if is_churen(tehai) {
        yaku.push(("純正九蓮宝燈".to_string(), 13));
    }

    // 九蓮宝燈
    if is_churen(tehai) {
        yaku.push(("九蓮宝燈".to_string(), 13));
    }

    // 四暗刻
    if is_suanko(tehai, fulo) {
        yaku.push(("四暗刻".to_string(), 13));
    }

    // 大三元
    if is_daisangen(tehai) {
        yaku.push(("大三元".to_string(), 13));
    }

    // 小四喜
    if is_shosushi(tehai) {
        yaku.push(("小四喜".to_string(), 13));
    }

    // 大四喜
    if is_daisushi(tehai) {
        yaku.push(("大四喜".to_string(), 13));
    }

    // 字一色
    if is_tsuiso(tehai) {
        yaku.push(("字一色".to_string(), 13));
    }

    // 清老頭
    if is_chinroto(tehai) {
        yaku.push(("清老頭".to_string(), 13));
    }

    // 緑一色
    if is_ryuiso(tehai) {
        yaku.push(("緑一色".to_string(), 13));
    }

    // 清一色
    if is_chiniso(tehai) {
        yaku.push(("清一色".to_string(), 6));
    }

    // 混一色
    if is_honiso(tehai) {
        yaku.push(("混一色".to_string(), 3));
    }    


    yaku
}


/// 上がり点を計算します
fn get_agari(fu: i32, yaku: &Vec<(String, i32)>) -> Agari {
    let mut agari = Agari::default();

    if yaku.len() == 0 {
        return agari;
    }

    // 基本点を算出する
    if yaku[0].1 == 0 {
        // 役満の場合
        agari.fu = 0;
        agari.ten = 8000;
    } else {
        agari.ten = fu * 2_i32.pow(yaku[0].1 as u32 + 2);
    }

    */

    yaku
}
