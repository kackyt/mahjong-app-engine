#[cfg(test)]
mod tests {
    use mahjong_core::{mahjong_generated::open_mahjong::{MentsuFlag, Mentsu, Pai, MentsuType, MentsuPai}, agari::add_machi_to_mentsu};

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

}

