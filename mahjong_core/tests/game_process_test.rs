use mahjong_core::mahjong_generated::open_mahjong::{GameStateT, PaiT};


#[test]
fn game_start_test() {
  let mut state = GameStateT::default();

  // 1人プレイのテスト
  state.create("test".as_bytes(), 1);
  state.shuffle();

  state.start();

  assert!(state.tsumo().is_ok(), "ツモ失敗");

  let mut player = state.get_player(0);

  for item in &player.tehai {
    print!("{}", item);
  }
  println!();
  println!("{}", player.tsumohai);

  assert_eq!(player.tehai_len, 13);
  assert_ne!(player.tsumohai, PaiT::default());

  state.sutehai(10);
  assert!(state.tsumo().is_ok(), "ツモ失敗");

  player = state.get_player(0);

  for item in &player.tehai {
    print!("{}", item);
  }
  println!();
  println!("{}", player.tsumohai);

  assert_eq!(player.tehai_len, 13);
  assert_ne!(player.tsumohai, PaiT::default());
}
