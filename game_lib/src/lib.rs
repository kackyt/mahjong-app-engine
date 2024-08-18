use std::{ffi::CStr, mem, os::raw::c_char};

use mahjong_core::{
    mahjong_generated::open_mahjong::{ActionType, GameStateT, PaiT, Player},
    play_log::PlayLog,
    shanten::PaiState,
};

#[no_mangle]
pub extern "C" fn get_work_mem_size() -> usize {
    mem::size_of::<GameStateT>()
}

#[no_mangle]
pub extern "C" fn get_player_mem_size() -> usize {
    mem::size_of::<Player>()
}

#[no_mangle]
pub unsafe extern "C" fn initialize(ptr: *mut GameStateT, title: *const c_char, player_len: u32) {
    let gamestate = ptr.as_mut().unwrap();
    let title_str = CStr::from_ptr(title);
    let title_slice = title_str.to_bytes();
    let mut play_log = PlayLog::new();

    gamestate.clone_from(&GameStateT::default());
    gamestate.create(title_slice, player_len, &mut play_log);
    gamestate.start(&mut play_log);
}

#[no_mangle]
pub unsafe extern "C" fn get_player_shanten(ptr: *mut GameStateT, player_index: usize) -> i32 {
    let gamestate = ptr.as_mut().unwrap();

    let player = gamestate.get_player(player_index);
    let mut tehai: Vec<PaiT> = player.tehai[0..(player.tehai_len as usize)]
        .iter()
        .cloned()
        .collect();

    if player.is_tsumo {
        tehai.push(player.tsumohai)
    }

    PaiState::from(&tehai).get_shanten(0)
}

#[no_mangle]
pub unsafe extern "C" fn do_action(
    ptr: *mut GameStateT,
    action_type: u32,
    player_index: usize,
    param: u32,
) {
    let gamestate = ptr.as_mut().unwrap();
    let mut play_log = PlayLog::new();

    let _ = gamestate.action(&mut play_log, ActionType(action_type), player_index, param);
}

#[no_mangle]
pub unsafe extern "C" fn get_player_state(
    ptr: *mut GameStateT,
    player_index: usize,
    ptr_player: *mut Player,
) {
    let gamestate = ptr.as_mut().unwrap();

    let player = gamestate.get_player(player_index);

    *ptr_player = player.pack();
}
