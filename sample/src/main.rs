use std::ffi::CString;

use anyhow::ensure;
use clap::Parser;
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use mahjong_core::{
    load_pailist::load_pailist,
    mahjong_generated::open_mahjong::{ActionType, GameStateT, PaiT},
    play_log::PlayLog,
    shanten::PaiState,
};

#[derive(Parser, Debug)]
#[command(author, about, version)]
struct Command {
    #[arg(short, long)]
    pai_list_file: Option<String>,
    #[arg(short, long, default_value_t = 0)]
    index: usize,
}

fn cmd() -> anyhow::Result<u32> {
    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => {
                    if c == '-' {
                        return Ok(10);
                    }

                    if c == '^' {
                        return Ok(11);
                    }

                    if c == '\\' {
                        return Ok(12);
                    }

                    if c == '0' {
                        return Ok(9);
                    }

                    if c >= '1' && c <= '9' {
                        return Ok(c as u32 - '1' as u32);
                    }

                    ensure!(false, "{} unknown code", c);
                }
                KeyCode::Backspace => {
                    return Ok(13);
                }

                _ => {
                    println!("unknown\r");
                }
            },
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Command::parse();
    let mut game_state: GameStateT = Default::default();
    let title: CString = CString::new("title")?;
    let keys = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '^', '¥',
    ];
    let mut play_log = PlayLog::new();

    enable_raw_mode()?;

    println!("initialize\r");

    game_state.create(title.as_bytes(), 1, &mut play_log);
    if let Some(pai_list) = args.pai_list_file {
        let hai_ids = load_pailist(pai_list, args.index)?;
        game_state.load(&hai_ids);
    } else {
        game_state.shuffle();
    }

    game_state.start(&mut play_log);

    loop {
        let _ = game_state.tsumo(&mut play_log);

        let player = game_state.get_player(0);

        player.tehai.iter().enumerate().for_each(|(idx, p)| {
            print!("{}[{}] ", p, keys[idx]);
        });
        if player.is_tsumo {
            print!("{}[BS]", player.tsumohai);
        }

        // シャンテン数を計算
        let mut tehai: Vec<PaiT> = player.tehai.iter().cloned().collect();
        tehai.push(player.tsumohai);

        let shanten = PaiState::from(&tehai).get_shanten(0);
        println!(" シャンテン数 {}\r", shanten);

        let command = cmd();

        if let Ok(sutehai) = command {
            let _ = game_state.action(&mut play_log, ActionType::ACTION_SUTEHAI, 0, sutehai);
            println!("\r");
        } else {
            println!("end\r");
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}
