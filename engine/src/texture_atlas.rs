use crate::Renderer;

pub struct TextureAtlas {
    pixels: Vec<u8>,
    sheet_width: u32,
    tile_size: u32,
    columns: u32,
    rows: u32,
}

impl TextureAtlas {
    pub fn from_file(path: &str, tile_size: u32) -> Self {
        let image = image::open(path)
            .unwrap_or_else(|error| panic!("failed to load texture atlas '{path}': {error}"))
            .into_rgba8();
        let sheet_width = image.width();
        let columns = sheet_width / tile_size;
        let rows = image.height() / tile_size;
        Self {
            pixels: image.into_raw(),
            sheet_width,
            tile_size,
            columns,
            rows,
        }
    }

    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    pub fn get_tile_pixels(&self, tile_index: usize) -> Vec<u8> {
        let column = tile_index as u32 % self.columns;
        let row = tile_index as u32 / self.columns;
        let origin_x = column * self.tile_size;
        let origin_y = row * self.tile_size;
        let mut output = vec![0u8; (self.tile_size * self.tile_size * 4) as usize];

        for y in 0..self.tile_size {
            let src_offset = ((origin_y + y) * self.sheet_width + origin_x) as usize * 4;
            let dst_offset = (y * self.tile_size) as usize * 4;
            let row_bytes = self.tile_size as usize * 4;
            output[dst_offset..dst_offset + row_bytes]
                .copy_from_slice(&self.pixels[src_offset..src_offset + row_bytes]);
        }

        output
    }

    pub fn extract_all_tiles(&self) -> Vec<Vec<u8>> {
        let count = (self.columns * self.rows) as usize;
        (0..count).map(|index| self.get_tile_pixels(index)).collect()
    }

    pub fn draw_tile(&self, renderer: &mut dyn Renderer, tile_index: usize, x: i32, y: i32) {
        let pixels = self.get_tile_pixels(tile_index);
        renderer.draw_sprite(x, y, self.tile_size, self.tile_size, &pixels);
    }
}
