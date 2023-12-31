use crate::chunks::FullChunk;
use crate::player::Player;

pub struct Server {
    chunks: Vec<FullChunk>,
    config: ServerConfig,
    players: Vec<Player>
}

pub struct ServerConfig {
    name: String,
}
