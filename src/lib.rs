use macroquad::prelude::*;
// This is a cross platform file reading function
use macroquad::file::load_string;

// This has to be async because of Macroquad
pub async fn load_tilemap(path: String) -> Tilemap {
    let file = load_string(&path).await.unwrap();
    // Make sure the file isn't empty
    if file.len() == 0 {
        panic!("Cannot load tilemap, file is empty!");
    }

    // The split between the tileset's file name and tiles
    let split: Vec<&str> = file.split('"').collect();

    // This section converts the file into a Vec<Vec<Vec<16>>>

    let mut tilemap: Vec<Vec<Vec<u16>>> = Vec::new();

    // Split the file into layers
    let layers: Vec<&str> = split[2].split('~').collect();
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

    let texture = load_texture(split[1]).await.unwrap();
    Tilemap {
        texture,
        tile_size: texture.height() as u16,
        tiles: tilemap,
    }
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

#[derive(Copy, Clone, PartialEq)]
pub struct TilemapRenderParams {
    // The minimum and maximum render positions in units of the tile size
    pub min_tile_render_pos: Vec2,
    pub max_tile_render_pos: Vec2,
    pub min_render_layer: usize,
    pub max_render_layer: usize,
}

// This is just so if you want to render the whole thing, you just say TilemapRenderParams::default()
impl Default for TilemapRenderParams {
    fn default() -> Self {
        Self {
            min_tile_render_pos: Vec2::ZERO,
            max_tile_render_pos: vec2(f32::MAX, f32::MAX),
            min_render_layer: 0,
            max_render_layer: usize::MAX,
        }
    }
}

impl Tilemap {
    pub fn render(&self, mut params: TilemapRenderParams) {
        // This just makes sure that the positions and ranges aren't too big or too small
        params.min_tile_render_pos = vec2(
            clamp(params.min_tile_render_pos.x, 0.0, self.tiles[0][0].len() as f32),
            clamp(params.min_tile_render_pos.y, 0.0, self.tiles[0].len() as f32),
        );
        params.max_tile_render_pos = vec2(
            clamp(params.max_tile_render_pos.x, 0.0, self.tiles[0][0].len() as f32),
            clamp(params.max_tile_render_pos.y, 0.0, self.tiles[0].len() as f32),
        );

        params.min_render_layer = clamp(params.min_render_layer, 0, self.tiles.len());
        params.max_render_layer = clamp(params.max_render_layer, 0, self.tiles.len());

        for z in params.min_render_layer..params.max_render_layer {
            for y in params.min_tile_render_pos.y as usize..params.max_tile_render_pos.y as usize {
                for x in params.min_tile_render_pos.x as usize..params.max_tile_render_pos.x as usize {
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