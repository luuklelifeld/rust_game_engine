pub struct TileMaterial {
    pub atlas_index: usize,
    pub walkable: bool,
}

pub static GRASS: TileMaterial = TileMaterial { atlas_index: 0, walkable: true };
pub static WATER: TileMaterial = TileMaterial { atlas_index: 1, walkable: false };
pub static STONE: TileMaterial = TileMaterial { atlas_index: 2, walkable: false };
pub static DIRT: TileMaterial = TileMaterial { atlas_index: 3, walkable: true };
