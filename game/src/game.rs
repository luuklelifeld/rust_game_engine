use engine::{GameLoop, Input, Key, Renderer, TextureAtlas};

use crate::player::Player;
use crate::tilemap::{TILE_SIZE, TileMap};

pub struct Game {
    map: TileMap,
    player: Player,
    camera_x: f32,
    camera_y: f32,
    tile_sprites: Vec<Vec<u8>>,
    player_sprite: Vec<u8>,
}

impl Game {
    pub fn new() -> Self {
        let map = TileMap::test_map();
        let player = Player::new(2.0 * TILE_SIZE as f32, 2.0 * TILE_SIZE as f32);

        let tile_atlas = TextureAtlas::from_file("game/assets/tiles.png", 16);
        let tile_sprites = tile_atlas.extract_all_tiles();

        let player_atlas = TextureAtlas::from_file("game/assets/player.png", 16);
        let player_sprite = player_atlas.get_tile_pixels(0);

        Self {
            map,
            player,
            camera_x: 0.0,
            camera_y: 0.0,
            tile_sprites,
            player_sprite,
        }
    }
}

impl GameLoop for Game {
    fn update(&mut self, input: &Input, delta_time: f32, screen_width: u32, screen_height: u32) {
        let mut direction_x: f32 = 0.0;
        let mut direction_y: f32 = 0.0;

        if input.is_held(Key::W) || input.is_held(Key::ArrowUp) {
            direction_y -= 1.0;
        }
        if input.is_held(Key::S) || input.is_held(Key::ArrowDown) {
            direction_y += 1.0;
        }
        if input.is_held(Key::A) || input.is_held(Key::ArrowLeft) {
            direction_x -= 1.0;
        }
        if input.is_held(Key::D) || input.is_held(Key::ArrowRight) {
            direction_x += 1.0;
        }

        // normalize diagonal
        let length = (direction_x * direction_x + direction_y * direction_y).sqrt();
        if length > 0.0 {
            direction_x /= length;
            direction_y /= length;
        }

        // move player with tile collision (axes checked independently for wall sliding)
        // check all 4 corners of the 16x16 sprite against tile walkability
        let candidate_x = self.player.x + direction_x * self.player.speed * delta_time;
        let left = (candidate_x / TILE_SIZE as f32) as u32;
        let right = ((candidate_x + TILE_SIZE as f32 - 1.0) / TILE_SIZE as f32) as u32;
        let top = (self.player.y / TILE_SIZE as f32) as u32;
        let bottom = ((self.player.y + TILE_SIZE as f32 - 1.0) / TILE_SIZE as f32) as u32;
        if self.map.is_walkable(left, top)
            && self.map.is_walkable(right, top)
            && self.map.is_walkable(left, bottom)
            && self.map.is_walkable(right, bottom)
        {
            self.player.x = candidate_x;
        }

        let candidate_y = self.player.y + direction_y * self.player.speed * delta_time;
        let left = (self.player.x / TILE_SIZE as f32) as u32;
        let right = ((self.player.x + TILE_SIZE as f32 - 1.0) / TILE_SIZE as f32) as u32;
        let top = (candidate_y / TILE_SIZE as f32) as u32;
        let bottom = ((candidate_y + TILE_SIZE as f32 - 1.0) / TILE_SIZE as f32) as u32;
        if self.map.is_walkable(left, top)
            && self.map.is_walkable(right, top)
            && self.map.is_walkable(left, bottom)
            && self.map.is_walkable(right, bottom)
        {
            self.player.y = candidate_y;
        }

        // clamp to map bounds
        let pixel_width = self.map.pixel_width() as f32;
        let pixel_height = self.map.pixel_height() as f32;
        self.player.x = self.player.x.clamp(0.0, pixel_width - TILE_SIZE as f32);
        self.player.y = self.player.y.clamp(0.0, pixel_height - TILE_SIZE as f32);

        // center camera on player, clamp to map edges
        let half_tile = TILE_SIZE as f32 / 2.0;
        self.camera_x = self.player.x + half_tile - screen_width as f32 / 2.0;
        self.camera_y = self.player.y + half_tile - screen_height as f32 / 2.0;
        self.camera_x = self
            .camera_x
            .clamp(0.0, (pixel_width - screen_width as f32).max(0.0));
        self.camera_y = self
            .camera_y
            .clamp(0.0, (pixel_height - screen_height as f32).max(0.0));
    }

    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.clear(0, 0, 0);

        let camera_x = self.camera_x as i32;
        let camera_y = self.camera_y as i32;
        let tile_size = TILE_SIZE as i32;
        let screen_width = renderer.width();
        let screen_height = renderer.height();

        // visible tile range
        let tile_start_x = (camera_x / tile_size).max(0) as u32;
        let tile_start_y = (camera_y / tile_size).max(0) as u32;
        let tile_end_x =
            ((camera_x + screen_width as i32) / tile_size + 1).min(self.map.width as i32) as u32;
        let tile_end_y =
            ((camera_y + screen_height as i32) / tile_size + 1).min(self.map.height as i32) as u32;

        for tile_y in tile_start_y..tile_end_y {
            for tile_x in tile_start_x..tile_end_x {
                let tile = self.map.get(tile_x, tile_y);
                let screen_x = tile_x as i32 * tile_size - camera_x;
                let screen_y = tile_y as i32 * tile_size - camera_y;
                renderer.draw_sprite(
                    screen_x,
                    screen_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    &self.tile_sprites[tile.material().atlas_index],
                );
            }
        }

        // draw player
        let player_x = self.player.x as i32 - camera_x;
        let player_y = self.player.y as i32 - camera_y;
        renderer.draw_sprite(player_x, player_y, TILE_SIZE, TILE_SIZE, &self.player_sprite);
    }
}
