use crate::chunks::FullChunk;

pub struct Server {
    chunks: Vec<FullChunk>,
    config: ServerConfig,
    players: Vec<Player>
}

pub struct ServerConfig {
    name: String,
}
