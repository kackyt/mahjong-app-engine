#[cfg(feature = "load-pailist")]
mod load_pailist_test {
    use std::path::PathBuf;

    use mahjong_core::{load_pailist, shanten::{PaiState, all_of_mentsu}};

    #[test]
    fn test_load_pailist() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/haiyamas-random.parquet");

        let ret = load_pailist::load_pailist(path, 0);

        assert!(ret.is_ok(), "成功");

        assert_eq!(ret.unwrap().len(), 136);
    }

    #[test]
    fn test_load_agari_tehai() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/agaris.parquet");

        let ret = load_pailist::load_agari_tehai(path, 3);

        assert!(ret.is_ok(), "成功");

        let pai_list = ret.unwrap();

        assert_eq!(pai_list.len(), 14);

        let mut pai_state = PaiState::from(&pai_list);

        let all_mentsu = all_of_mentsu(&mut pai_state);

        println!("{:?}\r", all_mentsu);

        assert_eq!(all_mentsu[0].len(), 5);
    }
}
