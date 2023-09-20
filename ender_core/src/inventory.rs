use crate::inventory::item::ItemStack;

mod item;

pub struct PlayerInventory {
    hotbar: InventoryRow,
    slots: [InventoryRow; 4],
}

pub struct ChestInventory {
    slots: [InventoryRow; 3],
}

pub struct DoubleChestInventory {
    slots: [InventoryRow; 6],
}

pub struct DropperInventory {
    slots: [[InventorySlot; 3]; 3],
}

pub struct HopperInventory {
    slots: [InventorySlot; 5],
}

pub struct FurnaceInventory {
    material: InventorySlot,
    smelted: InventorySlot,
    fuel: InventorySlot,
}

pub struct ComposterInventory {
    slot: InventorySlot,
}

pub struct InventoryRow {
    slots: [InventorySlot; 9],
}

pub enum InventorySlot {
    Empty,
    Filled(ItemStack),
}
