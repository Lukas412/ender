pub struct StairDirection {
    celestial: CelestialDirection,
    slice: SlabDirection,
}

pub enum SlabDirection {
    Upper,
    Lower,
}

pub enum CelestialDirection {
    North,
    East,
    South,
    West,
}

pub enum BlockDirection {
    North,
    East,
    South,
    West,
    Top,
    Bottom,
}

pub enum StraightDirection {
    NorthToSouth,
    EastToWest,
}

pub enum CurveDirection {
    NorthToEast,
    EastToSouth,
    SouthToWest,
    WestToNorth,
}

pub struct CornerDirection {
    pointed_to: CelestialDirection,
}
