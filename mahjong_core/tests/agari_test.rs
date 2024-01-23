#[cfg(feature = "load-pailist")]
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mahjong_core::{mahjong_generated::open_mahjong::{MentsuFlag, Mentsu, Pai, MentsuType, MentsuPai, GameStateT}, agari::{add_machi_to_mentsu, AgariBehavior}, load_pailist, shanten::{PaiState, all_of_mentsu}};

    #[test]
    fn test_add_machi_to_mentsu() {
        let mentsu = vec![
            vec![
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(5, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(6, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(7, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(9, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
        ];

        let p = Pai::new(2, 0, false, false, false);

        let result = add_machi_to_mentsu(&mentsu, &p);
        
        assert_eq!(result, vec![
            vec![
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_AGARI),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(5, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(6, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(7, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(8, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(9, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_AGARI),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
        ]);
    }


    #[test]
    fn test_add_machi_to_mentsu2() {
        let mentsu = vec![
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_KOUTSU),
                Mentsu::new(&[
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(5, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(6, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
        ];

        let p = Pai::new(2, 0, false, false, false);

        let result = add_machi_to_mentsu(&mentsu, &p);
        
        assert_eq!(result, vec![
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_AGARI),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_KOUTSU),
                Mentsu::new(&[
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(5, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(6, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_AGARI),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
            vec![
                Mentsu::new(&[
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(4, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
                Mentsu::new(&[
                    MentsuPai::new(1, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(2, 0, MentsuFlag::FLAG_AGARI),
                    MentsuPai::new(3, 0, MentsuFlag::FLAG_NONE),
                    MentsuPai::new(0, 0, MentsuFlag::FLAG_NONE),
                ], 3, MentsuType::TYPE_SHUNTSU),
            ],
        ]);
    }

    #[test]
    fn test_agari_ten() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/agaris.parquet");
        let game_state = GameStateT::default();

        let ret = load_pailist::load_agari_tehai(path, 0);

        assert!(ret.is_ok(), "成功");

        let parquet = ret.unwrap();
        println!("{:?}\r", parquet);

        // assert_eq!(parquet.tehai.len(), 14);

        let mut pai_state = PaiState::from_with_fulo(&parquet.tehai, &parquet.fulo);

        let all_mentsu = all_of_mentsu(&mut pai_state);
        let all_mentsu_w_machi = add_machi_to_mentsu(&all_mentsu, &parquet.machihai.pack());

        println!("{:?}\r", all_mentsu);
        println!("{:?}\r", all_mentsu_w_machi);

        let mentsu = all_mentsu_w_machi[0].clone();
        println!("{:?}\r", mentsu);


        let agari_state = game_state.get_agari(&mentsu, &parquet.fulo);
        let yakus = agari_state.get_yaku_list();
        let agari = agari_state.get_agari(&yakus);

        println!("{:?}\r", agari_state);
        println!("{:?}\r", agari);
        println!("{:?}\r", yakus);

        assert_eq!(agari.fu, parquet.fu);
        assert_eq!(agari.han, parquet.han);
        assert_eq!(agari.score, parquet.score);
    }


}

