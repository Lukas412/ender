use crate::blocks::red_stone::observer::ObserverBlock;
use crate::blocks::red_stone::piston::PistonBlock;
use crate::blocks::red_stone::powered::PoweredBlock;
use crate::number::Bounded15;

pub enum RedStoneBlock {
    Powered(PoweredBlock),
    Piston(PistonBlock),
    Observer(ObserverBlock),
}

mod powered {
    use crate::number::{Bounded1To15};

    pub struct PoweredBlock {
        block_type: PoweredBlockType,
        state: Bounded1To15,
    }

    pub enum PoweredBlockType {}
}

mod piston {
    use crate::blocks::red_stone::ActivationState;
    use crate::blocks::directions::BlockDirection;

    pub struct PistonBlock {
        piston_type: PistonType,
        direction: BlockDirection,
        state: ActivationState,
    }

    pub enum PistonType {
        Normal,
        Sticky,
    }
}

mod observer {
    use crate::blocks::red_stone::ActivationState;
    use crate::blocks::directions::BlockDirection;

    pub struct ObserverBlock {
        direction: BlockDirection,
        state: ActivationState,
    }
}

mod dust {
    use crate::blocks::red_stone::RedStoneState;
    use crate::blocks::directions::{CornerDirection, CurveDirection, StraightDirection};

    pub struct RedStoneDustBlock {
        direction: RedStoneDustDirection,
        state: RedStoneState,
    }

    pub enum RedStoneDustDirection {
        None,
        All,
        Straight(StraightDirection),
        Curve(CurveDirection),
        Corner(CornerDirection),
    }
}

pub enum ActivationState {
    Deactivated,
    Activated,
}

pub enum RedStoneState {
    Deactivated,
    Activated(Bounded15),
}