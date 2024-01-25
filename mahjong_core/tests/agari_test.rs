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
        let mut game_state = GameStateT::default();

        let ret = load_pailist::load_agari_tehai(path, 1);

        assert!(ret.is_ok(), "成功");

        let parquet = ret.unwrap();

        game_state.copy_dora(&parquet.dora);
        game_state.copy_uradora(&parquet.uradora);

        // assert_eq!(parquet.tehai.len(), 14);

        let mut pai_state = PaiState::from(&parquet.tehai);

        let all_mentsu = all_of_mentsu(&mut pai_state, parquet.fulo.len());
        let all_mentsu_w_machi = add_machi_to_mentsu(&all_mentsu, &parquet.machipai.pack());

        let agari = game_state.get_best_agari(0, &all_mentsu_w_machi, &parquet.fulo, parquet.nukidora as usize).unwrap();

        assert_eq!(agari.fu, parquet.fu);
        assert_eq!(agari.han, parquet.han);
        assert_eq!(agari.score, parquet.score);
    }


}

