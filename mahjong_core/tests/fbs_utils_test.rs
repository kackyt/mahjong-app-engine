use mahjong_core::{
    fbs_utils::TakuControl,
    mahjong_generated::open_mahjong::{TakuT, PaiT},
};

#[test]
fn bahai_shuffle_test() {
    let bahai = TakuT::create_shuffled();

    let target = PaiT {
        pai_num: 30,
        id: 2,
        is_tsumogiri: false,
        is_riichi: false,
        is_nakare: false,
    };

    assert!(
        bahai.search(&target).is_ok(),
        "シャッフルした牌が見つからない"
    );

    assert!(bahai.get(135).is_ok(), "雀卓の牌を取得できない");
}
