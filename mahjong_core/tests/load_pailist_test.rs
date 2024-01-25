#[cfg(feature = "load-pailist")]
mod load_pailist_test {
    use std::path::PathBuf;

    use mahjong_core::{load_pailist, shanten::{PaiState, all_of_mentsu}};

    #[test]
    fn test_load_pailist() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/paiyamas-random.parquet");

        let ret = load_pailist::load_pailist(path, 0);

        assert!(ret.is_ok(), "成功");

        assert_eq!(ret.unwrap().len(), 136);
    }

    #[test]
    fn test_load_agari_tehai() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/agaris.parquet");

        let ret = load_pailist::load_agari_tehai(path, 5);

        assert!(ret.is_ok(), "成功");

        let parquet = ret.unwrap();

        assert_eq!(parquet.tehai.len(), 14);

        let mut pai_state = PaiState::from(&parquet.tehai);

        let all_mentsu = all_of_mentsu(&mut pai_state, parquet.fulo.len());

        assert_eq!(all_mentsu[0].len(), 5);
    }
}
