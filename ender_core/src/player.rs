use uuid::Uuid;
use crate::inventory::PlayerInventory;

pub struct Player {
    name: String,
    uuid: Uuid,
    inventory: PlayerInventory,
}
