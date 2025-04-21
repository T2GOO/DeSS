use serde::{Deserialize, Serialize};

//#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub data: Vec<u8>,
    pub created_at: u64,
    pub size: usize,
}
//#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub name: String,
    pub chunks: Vec<Chunk>,
    pub created_at: u64,
}