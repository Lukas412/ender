use crate::number::{Bounded1To16, Bounded1To64};

pub enum ItemStack {
    Single(SingleItemStack),
    Quarter(QuarterItemStack),
    Full(FullItemStack),
}

pub enum SingleItemStack {
    MineCart,
    Boat,
    WaterBucket,
    LavaBucket,
    EnchantedBook {
        enchantment: Enchantment
    },
    SignedBook {
        name: String
    },
    Tool {
        tool_type: ToolType,
        material: ToolMaterial,
    },
    Armor {
        armor_type: ArmorType,
        material: ArmorMaterial,
    },
}

pub enum ToolType {
    Pickaxe,
    Axe,
    Sword,
    Shovel,
    Hoe,
}

pub enum ToolMaterial {
    Wood,
    Stone,
    Iron,
    Gold,
    Diamond,
    Netherite,
}

pub enum ArmorType {
    Hat,
    ChestPlate,
    Pants,
    Boots,
}

pub enum ArmorMaterial {
    Leather,
    Chain,
    Iron,
    Gold,
    Diamond,
    Netherite,
}

pub enum Enchantment {}

pub struct QuarterItemStack {
    item_type: QuarterItemStackType,
    amount: Bounded1To16,
}

pub enum QuarterItemStackType {
    Egg,
    EnderPerl,
    EmptyBucket,
}

pub struct FullItemStack {
    item_type: FullItemStackType,
    amount: Bounded1To64,
}

pub enum FullItemStackType {}
