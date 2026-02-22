mod game;
mod player;
mod tile_material;
mod tilemap;

use engine::EngineConfig;
use game::Game;

fn main() {
    engine::run(
        EngineConfig {
            width: 320,
            height: 240,
        },
        || Game::new(),
    );
}
