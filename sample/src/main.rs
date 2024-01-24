use std::ffi::CString;

use clap::Parser;
use anyhow::ensure;
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}, event::{Event, KeyCode, read}};
use mahjong_core::{mahjong_generated::open_mahjong::{GameStateT, PaiT, ActionType}, load_pailist::load_pailist};

#[derive(Parser, Debug)]
#[command(author, about, version)]
struct Command {
    #[arg(short, long)]
    pai_list_file: Option<String>,
    #[arg(short, long, default_value_t=0)]
    index: usize
}

fn hai_to_str(p: &PaiT) -> String {
    let colors = ["m", "p", "s", "z"];

    let num = p.pai_num;

    let suit = (num / 9) as usize;

    format!("{}{}", colors[suit], num % 9)
}

fn cmd() -> anyhow::Result<u32> {
    loop {
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char(c) => {
                        if c == '-' {
                            return Ok(10);
                        }

                        if c == '^' {
                            return Ok(11);
                        }

                        if c == '\\' {
                            return Ok(12)
                        }

                        if c == '0' {
                            return Ok(9);
                        }

                        if c >= '1' && c <= '9' {
                            return Ok(c as u32 - '1' as u32);
                        }

                        ensure!(false, "{} unknown code", c);
                    },
                    KeyCode::Backspace => {
                        return Ok(13);    
                    },

                    _ => {
                        println!("unknown\r");
                    }
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
    let keys =['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '^', '¥'];

    enable_raw_mode()?;

    println!("initialize\r");

    game_state.create(title.as_bytes(), 1);
    if let Some(pai_list) = args.pai_list_file {
        let hai_ids = load_pailist(pai_list, args.index)?;
        game_state.load(&hai_ids);
    } else {
        game_state.shuffle();
    }

    game_state.start();

    loop {
        let _ = game_state.tsumo();

        let player = game_state.get_player(0);

        player.tehai.iter().enumerate().for_each(|(idx, p)| {
            print!("{}[{}] ", hai_to_str(p), keys[idx]);
        });
        if player.is_tsumo {
            print!("{}[BS]", player.tsumohai);
        }
        println!("\r");

        let command = cmd();

        if let Ok(sutehai) = command {
            let _ = game_state.action(ActionType::ACTION_SUTEHAI, 0, sutehai);
            println!("\r");
        } else {
            println!("end\r");
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}
