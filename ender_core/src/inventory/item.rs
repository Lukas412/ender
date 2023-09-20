use crate::number::{Bounded1To16, Bounded1To64};

pub enum ItemStack {
    Single(SingleItemStack),
    Quarter(QuarterItemStack),
    Full(FullItemStack),
}

pub enum SingleItemStack {}

pub struct QuarterItemStack {
    item_type: QuarterItemStackType,
    amount: Bounded1To16,
}

pub enum QuarterItemStackType {}

pub struct FullItemStack {
    item_type: FullItemStackType,
    amount: Bounded1To64,
}

pub enum FullItemStackType {}
