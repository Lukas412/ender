pub use any::AnyBlock;

mod any {
    use crate::blocks::slice_able::SliceAbleBlock;

    pub enum AnyBlock {
        Air,
        SliceAble(SliceAbleBlock),
    }
}

mod slice_able {
    use crate::{SlabDirection, StairDirection};

    pub struct SliceAbleBlock {
        block_type: SliceAbleBlockType,
        direction: SliceDirection,
    }

    pub enum SliceAbleBlockType {
        CobbleStone,
        Stone,
        SmoothStone,
        Andesite,
        PolishedAndesite,
        Diorite,
        PolishedDiorite,
        Granite,
        PolishedGranite,
        Sandstone,
        ChiseledSandstone,
        SmoothSandstone,
        RedSandstone,
        ChiseledRedSandstone,
        SmoothRedSandstone,
    }

    pub enum SliceDirection {
        Full,
        Slab(SlabDirection),
        Stair(StairDirection),
    }
}

mod red_stone;
pub mod directions;
