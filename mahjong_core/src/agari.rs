use crate::{mahjong_generated::open_mahjong::{PaiT, MentsuT, Mentsu, Pai}, shanten::PaiState};


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

/* pub fn add_machi_to_mentsu(mentsu: &mut Vec<Vec<Mentsu>>, machihai: &Pai) -> Vec<Vec<Mentsu>> {
    let mut ret: Vec<Vec<Mentsu>> = Vec::new();


    mentsu.into_iter().for_each(|item| {
        item.into_iter().for_each(|item2| {
            let mut mentsu = item2.clone();

            mentsu.pai_list().into_iter().for_each(|item3| {
                if item3.pai_num() == machihai.pai_num() {
                    mentsu.push(Mentsu::from(machihai));
                }
            });
            mentsu.push(Mentsu::from(machihai));
            ret.push(mentsu);
        });
        let mut mentsu = item.clone();
        mentsu.push(Mentsu::from(machihai));
        ret.push(mentsu);
    });

    ret
}
 */

pub fn add_machi_to_mentsu(mentsu: &Vec<Vec<Vec<(i32, bool)>>>, p: i32) -> Vec<Vec<Vec<(i32, bool)>>> {
    let mut result = Vec::new();

    // 各mentsu_vecに対して処理を行う
    for mentsu_vec in mentsu.iter() {
        let mut positions = Vec::new();

        for (index, mentsu_subvec) in mentsu_vec.iter().enumerate() {
            for (index2, mentsu_sub) in mentsu_subvec.iter().enumerate() {
                if mentsu_sub.0 == p {
                    positions.push((index, index2));
                    break;
                }
            }
        }

        if !positions.is_empty() {
            // すべての組み合わせを生成
            for &pos in &positions {
                let mut temp_vecs = vec![Vec::new()];
                for new_vec in &mut temp_vecs {
                    for (index, mentsu_subvec) in mentsu_vec.iter().enumerate() {
                        let updated_subvec = if pos.0 == index {
                            mentsu_subvec.iter().enumerate().map(|(idx, &(num, _))| (num, idx == pos.1)).collect()
                        } else {
                            mentsu_subvec.clone()
                        };
                        new_vec.push(updated_subvec);
                    }
                }
                result.extend(temp_vecs);
            }
        } else {
            result.push(mentsu_vec.clone());
        }
    }

    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_machi_to_mentsu() {
        let mut mentsu = vec![
            vec![
                vec![(1, false), (2, false), (3, false)],
                vec![(4, false), (5, false), (6, false)],
            ],
            vec![
                vec![(7, false), (8, false), (9, false)],
                vec![(1, false), (2, false), (3, false)],
            ],
        ];

        let result = add_machi_to_mentsu(&mut mentsu, 2);
        
        assert_eq!(result, vec![
            vec![vec![(1, false), (2, true), (3, false)], vec![(4, false), (5, false), (6, false)]],
            vec![vec![(7, false), (8, false), (9, false)], vec![(1, false), (2, true), (3, false)]],
        ]);
    }



    #[test]
    fn test_add_machi_to_mentsu2() {
        let mut mentsu = vec![
            vec![
                vec![(1, false), (2, false), (2, false)],
                vec![(4, false), (5, false), (6, false)],
            ],
            vec![
                vec![(7, false), (8, false), (2, false)],
                vec![(1, false), (2, false), (3, false)],
            ],
        ];

        let result = add_machi_to_mentsu(&mut mentsu, 2);
        
        assert_eq!(result, vec![
            vec![vec![(1, false), (2, true), (2, false)], vec![(4, false), (5, false), (6, false)]],
            vec![vec![(7, false), (8, false), (2, true)], vec![(1, false), (2, false), (3, false)]],
            vec![vec![(7, false), (8, false), (2, false)], vec![(1, false), (2, true), (3, false)]],
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
