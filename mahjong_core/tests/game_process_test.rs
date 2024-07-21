use mahjong_core::{
    mahjong_generated::open_mahjong::{GameStateT, PaiT},
    play_log,
};

#[test]
fn game_start_test() {
    let mut state = GameStateT::default();
    let mut play_log = play_log::PlayLog::new();

    // 1人プレイのテスト
    state.create("test".as_bytes(), 1, &mut play_log);
    state.shuffle();

    state.start(&mut play_log);

    assert!(state.tsumo(&mut play_log).is_ok(), "ツモ失敗");

    let mut player = state.get_player(0);

    for item in &player.tehai {
        print!("{}", item);
    }
    println!();
    println!("{}", player.tsumohai);

    assert_eq!(player.tehai_len, 13);
    assert_ne!(player.tsumohai, PaiT::default());

    state.sutehai(&mut play_log, 10, false);
    assert!(state.tsumo(&mut play_log).is_ok(), "ツモ失敗");

    player = state.get_player(0);

    for item in &player.tehai {
        print!("{}", item);
    }
    println!();
    println!("{}", player.tsumohai);

    assert_eq!(player.tehai_len, 13);
    assert_ne!(player.tsumohai, PaiT::default());
}
