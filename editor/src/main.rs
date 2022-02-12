use tile::*;
use macroquad::prelude::*;

#[macroquad::main("Tile Editor - v0.0.0")]
async fn main() {
    let tilemap = Tilemap {
        texture: load_texture("tileset.png").await.unwrap(),
        tile_size: 16,
        tiles: load_tilemap("current.tilemap".to_string()).await,
    };
    loop {
        clear_background(BLACK);
        tilemap.render(TilemapRenderParams::default());
        next_frame().await
    }
}