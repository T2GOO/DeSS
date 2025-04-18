use crate::storage::BlockStore;
use crate::security::CryptoEngine;

#[test]
fn test_store_and_load_block() {
    let path = "tmp/test_db_123";
    let crypto = CryptoEngine::new_from_key(&[0u8; 32]);

    let store = BlockStore::new(path, crypto).unwrap();
    store.store_block("block1", b"Hello").unwrap();

    let result = store.load_block("block1").unwrap();
    assert_eq!(result, b"Hello");
}
