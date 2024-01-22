use crate::{mahjong_generated::open_mahjong::{PaiT, MentsuT}, shanten::PaiState};


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
