use mahjong_core::{
    fbs_utils::TakuControl,
    mahjong_generated::open_mahjong::{TakuT, PaiT},
};

#[test]
fn taku_shuffle_test() {
    let taku = TakuT::create_shuffled();

    let target = PaiT {
        pai_num: 30,
        id: 2,
        is_tsumogiri: false,
        is_riichi: false,
        is_nakare: false,
    };

    assert!(
        taku.search(&target).is_ok(),
        "シャッフルした牌が見つからない"
    );

    assert!(taku.get(135).is_ok(), "雀卓の牌を取得できない");
}

#[test]
fn taku_get_range_test() {
    let taku = TakuT::create_shuffled();

    let r = taku.get_range(0..13);

    assert!(r.is_ok(), "成功");

    assert_eq!(r.unwrap().len(), 13);

    let r2 = taku.get_range(14..40);

    assert!(r2.is_ok(), "成功");

    assert_eq!(r2.unwrap().len(), 26);


    let r3 = taku.get_range(34..130);

    assert!(r3.is_ok(), "成功");

    assert_eq!(r3.unwrap().len(), 96);
}

