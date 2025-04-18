mod storage;
mod security;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use storage::block_store::BlockStore;
    use storage::chunker;
    use storage::chunker::chunk_data;
    //ANCHOR STORAGE
    #[test]
    fn test_chunking_works() {
        let data = b"0123456789";
        let chunks = chunk_data(data, 3).unwrap();
        assert_eq!(chunks.len(), 4);
    }
    #[test]
    fn test_chunking_too_long() {
        let data = b"0123456789";
        let chunks = chunk_data(data, 50).unwrap();
        assert!(chunks.len() == 1);
    }
    #[test]
    fn test_read_pdf_as_bytes() {
        let path_to_file = "data_test/test.pdf";
        let data = chunker::read_file_to_bytes(path_to_file).unwrap();
        assert!(!data.is_empty());
    }
    #[test]
    fn test_read_exe_as_bytes_failed() {
        let path_to_file = "data_test/test.exe";
        let result = chunker::read_file_to_bytes(path_to_file);
        assert!(result.is_err());
    }
    #[test]
    fn test_chunking_file() {
        let path_to_file = "data_test/test.pdf";
        let chunk_size = 1024;
        let chunks = chunker::chunk_file(path_to_file, chunk_size).unwrap();
        assert!(!chunks.is_empty());
        println!("Number of chunks: {}", chunks.len());
        for chunk in &chunks {
            assert_eq!(chunk.len() <= chunk_size, true);
        }
    }
    #[test]
    fn test_db() {
        use security::crypto::CryptoEngine;
        let path = "data_test/test.db";
        let key = [0u8; 32]; // ONLY FOR TESTING

        let crypto = CryptoEngine::new_from_key(&key);
        let state = BlockStore::new(path, crypto);
        println!("BlockStore created: {:?}", state);
        assert_eq!(state.is_ok(), true);
        let block_store = state.unwrap();

        let data = b"0123456789";
        let state = block_store.store_block("test_key_to_load", data);
        assert_eq!(state.is_ok(), true);

        block_store.block_summary().unwrap();

        let result = block_store.load_block("test_key_to_load");
        println!("Loaded block: {:?}", result);
        assert_eq!(result.is_ok(), true);

        let state = block_store.delete_block("test_key_to_load");
        assert_eq!(state.is_ok(), true);
    }
    #[test]
    fn test_fail_create_block_store() {
        use security::crypto::CryptoEngine;
        let path = "data_test/test.db";

        //? Unsecure Crypto Engine
        let crypto = CryptoEngine::new();
        let block_store = BlockStore::new(path, crypto);
        assert_eq!(block_store.is_err(), true);
        println!("Failed to create BlockStore: {:?}", block_store.err().unwrap());
    }
}