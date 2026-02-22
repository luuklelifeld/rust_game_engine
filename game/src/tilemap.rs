use crate::tile_material::{self, TileMaterial};

pub const TILE_SIZE: u32 = 16;

#[derive(Clone, Copy)]
pub enum Tile {
    Grass,
    Water,
    Stone,
    Dirt,
}

impl Tile {
    pub fn material(self) -> &'static TileMaterial {
        match self {
            Tile::Grass => &tile_material::GRASS,
            Tile::Water => &tile_material::WATER,
            Tile::Stone => &tile_material::STONE,
            Tile::Dirt => &tile_material::DIRT,
        }
    }
}

pub struct TileMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

impl TileMap {
    pub fn test_map() -> Self {
        let width = 30;
        let height = 20;
        let mut tiles = vec![Tile::Grass; (width * height) as usize];

        // stone border
        for x in 0..width {
            tiles[x as usize] = Tile::Stone;
            tiles[((height - 1) * width + x) as usize] = Tile::Stone;
        }
        for y in 0..height {
            tiles[(y * width) as usize] = Tile::Stone;
            tiles[(y * width + width - 1) as usize] = Tile::Stone;
        }

        // water lake in center
        for y in 8..13 {
            for x in 12..19 {
                tiles[(y * width + x) as usize] = Tile::Water;
            }
        }

        // dirt path near the lake
        for x in 8..12 {
            tiles[(10 * width + x) as usize] = Tile::Dirt;
        }
        for y in 6..14 {
            tiles[(y * width + 8) as usize] = Tile::Dirt;
        }

        Self { width, height, tiles }
    }

    pub fn get(&self, x: u32, y: u32) -> Tile {
        self.tiles[(y * self.width + x) as usize]
    }

    pub fn pixel_width(&self) -> u32 {
        self.width * TILE_SIZE
    }

    pub fn pixel_height(&self) -> u32 {
        self.height * TILE_SIZE
    }

    pub fn is_walkable(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.get(x, y).material().walkable
    }
}
