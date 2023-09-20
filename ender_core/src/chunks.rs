use crate::blocks::AnyBlock;
use crate::entities::AnyEntity;

const CHUNK_SIZE: usize = 16;
const CHUNK_HEIGHT: usize = 256;

pub struct FullChunk {
    blocks: [[[AnyBlock; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT],
    entities: Vec<AnyEntity>,
    biome: Biome,
    sub_biomes: [SubBiome; CHUNK_HEIGHT / CHUNK_SIZE]
}

pub enum Biome {}

pub enum SubBiome {
    None
}
