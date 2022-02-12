use macroquad::prelude::*;
// This is a cross platform file reading function
use macroquad::file::load_string;

// This has to be async because of Macroquad
pub async fn load_tilemap(path: String) -> Vec<Vec<Vec<u16>>> {
    let file = load_string(&path).await.unwrap();
    // Make sure the file isn't empty
    if file.len() == 0 {
        return Vec::new();
    }

    // This section converts the file into a Vec<Vec<Vec<16>>>

    let mut tilemap: Vec<Vec<Vec<u16>>> = Vec::new();

    // Split the file into layers
    let layers: Vec<&str> = file.split('~').collect();
    for layer in layers.iter() {
        let mut tile_rows: Vec<Vec<u16>> = Vec::new();
        // Split the layer into rows
        let rows: Vec<&str> = layer.split('/').collect();
        for row in rows.iter() {
            let mut tile_row: Vec<u16> = Vec::new();
            // Split the row into tiles
            let tiles: Vec<&str> = row.split(',').collect();
            for tile in tiles {
                tile_row.push(tile.parse::<u16>().unwrap());
            }
            tile_rows.push(tile_row);
        }
        tilemap.push(tile_rows);
    }

    tilemap
}

#[derive(Clone, PartialEq)]
pub struct Tilemap {
    pub texture: Texture2D,
    pub tile_size: u16,
    pub tiles: Vec<Vec<Vec<u16>>>,
}

impl Default for Tilemap {
    fn default() -> Self {
        Self {
            texture: Texture2D::empty(),
            tile_size: 16,
            tiles: Vec::new(),
        }
    }
}

impl Tilemap {
    pub fn render(&self, mut min_render_tile_pos: Vec2, mut max_render_tile_pos: Vec2, mut min_render_layer: usize, mut max_render_layer: usize) {
        // This just makes sure that the positions and ranges aren't too big or too small
        min_render_tile_pos = vec2(
            clamp(min_render_tile_pos.x, 0.0, self.tiles[0][0].len() as f32),
            clamp(min_render_tile_pos.y, 0.0, self.tiles[0].len() as f32),
        );
        max_render_tile_pos = vec2(
            clamp(max_render_tile_pos.x, 0.0, self.tiles[0][0].len() as f32),
            clamp(max_render_tile_pos.y, 0.0, self.tiles[0].len() as f32),
        );

        min_render_layer = clamp(min_render_layer, 0, self.tiles.len());
        max_render_layer = clamp(max_render_layer, 0, self.tiles.len());

        for z in min_render_layer..max_render_layer {
            for y in min_render_tile_pos.y as usize..max_render_tile_pos.y as usize {
                for x in min_render_tile_pos.x as usize..max_render_tile_pos.x as usize {
                    // Don't render if it's not zero
                    if self.tiles[z][y][x] != 0 {
                        draw_texture_ex(
                            self.texture,
                            x as f32 * self.tile_size as f32,
                            y as f32 * self.tile_size as f32,
                            WHITE,
                            DrawTextureParams {
                                source: Some(Rect {
                                    x: ((self.tiles[z][y][x] - 1) * self.tile_size) as f32,
                                    y: 0.0,
                                    w: self.tile_size as f32,
                                    h: self.tile_size as f32,
                                }),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
    }
}