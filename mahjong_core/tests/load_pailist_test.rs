#[cfg(feature = "load-pailist")]
#[test]
fn test_load_pailist() {
    use std::path::PathBuf;

    use mahjong_core::load_pailist;

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/haiyamas-random.parquet");

    let ret = load_pailist::load_pailist(path, 0);

    assert!(ret.is_ok(), "成功");

    assert_eq!(ret.unwrap().len(), 136);
}
