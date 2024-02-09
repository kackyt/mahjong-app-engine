use mahjong_core::{mahjong_generated::open_mahjong::PaiT, shanten::PaiState};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
    path::Path,
};

fn parse_testcase(path: &Path) -> io::Result<Vec<(Vec<PaiT>, i32)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut pai_vec: Vec<(Vec<PaiT>, i32)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|word| word.parse().ok())
            .collect();

        let mut pai_array: Vec<PaiT> = Vec::new();

        for i in 0..14 {
            let p = PaiT {
                pai_num: numbers[i] as u8,
                id: 1,
                is_tsumogiri: false,
                is_riichi: false,
                is_nakare: false
            };

            pai_array.push(p);
        }

        let min_shanten = numbers[14]; // .min(numbers[15]).min(numbers[16]);

        pai_vec.push((pai_array, min_shanten));
    }

    Ok(pai_vec)
}

#[test]
fn calc_hon_shanten_test() -> Result<(), Error> {
    let filepath = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/p_hon_10000.txt");
    let case1 = parse_testcase(&filepath)?;
    let mut lines = 1;

    for case in case1 {
        let mut state = PaiState::from(&case.0);
        println!("case {}", lines);

        assert_eq!(state.get_shanten(0), case.1);
        lines += 1;
    }

    Ok(())
}

#[test]
fn calc_koku_shanten_test() -> Result<(), Error> {
    let filepath = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/p_koku_10000.txt");
    let case1 = parse_testcase(&filepath)?;

    for case in case1 {
        let mut state = PaiState::from(&case.0);

        assert_eq!(state.get_shanten(0), case.1);
    }

    Ok(())
}

#[test]
fn calc_normal_shanten_test() -> Result<(), Error> {
    let filepath = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/p_normal_10000.txt");
    let case1 = parse_testcase(&filepath)?;

    for case in case1 {
        let mut state = PaiState::from(&case.0);

        assert_eq!(state.get_shanten(0), case.1);
    }

    Ok(())
}

#[test]
fn calc_tin_shanten_test() -> Result<(), Error> {
    let filepath = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/p_tin_10000.txt");
    let case1 = parse_testcase(&filepath)?;

    for case in case1 {
        let mut state = PaiState::from(&case.0);

        assert_eq!(state.get_shanten(0), case.1);
    }

    Ok(())
}
