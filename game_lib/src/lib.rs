use std::{mem, os::raw::c_char, ffi::CStr};

use mahjong_core::{mahjong_generated::open_mahjong::{GameStateT, Player, ActionType, PaiT}, shanten::PaiState};


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

    gamestate.clone_from(&GameStateT::default());
    gamestate.create(title_slice, player_len);
    gamestate.start()
}

#[no_mangle]
pub unsafe extern "C" fn get_player_shanten(ptr: *mut GameStateT, player_index: usize) -> i32 {
    let gamestate = ptr.as_mut().unwrap();

    let player = gamestate.get_player(player_index);
    let mut tehai: Vec<PaiT> = player.tehai[0..(player.tehai_len as usize)].iter().cloned().collect();


    if player.is_tsumo {
        tehai.push(player.tsumohai)
    }

    PaiState::from(&tehai).get_shanten()
}

#[no_mangle]
pub unsafe extern "C" fn do_action(ptr: *mut GameStateT, action_type: u32, player_index: usize, param: u64) {
    let gamestate = ptr.as_mut().unwrap();

    let _ = gamestate.action(ActionType(action_type), player_index, param);
}

#[no_mangle]
pub unsafe extern "C" fn get_player_state(ptr: *mut GameStateT, player_index: usize, ptr_player: *mut Player) {
    let gamestate = ptr.as_mut().unwrap();

    let player = gamestate.get_player(player_index);

    *ptr_player = player.pack();
}
