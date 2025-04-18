use std::fs;

const EXTENSION_NOT_SUPPORTED: [&str;4] = ["exe", "dll", "bin", "so"];

pub fn chunk_data(data: &[u8], chunk_size: usize) -> Result<Vec<&[u8]>, std::io::Error> {
    if chunk_size == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "chunk size must be greater than 0",
        ));
    }
    Ok(data.chunks(chunk_size).collect())
}

pub fn read_file_to_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let extension = path.split('.').last().unwrap_or("");
    if EXTENSION_NOT_SUPPORTED.contains(&extension) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "file extension not supported",
        ));
    }
    fs::read(path)
}

pub fn chunk_file(path: &str, chunk_size: usize) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let data = read_file_to_bytes(path)?;
    let chunks = chunk_data(&data, chunk_size)?;
    Ok(chunks.iter().map(|&chunk| chunk.to_vec()).collect())
}
