use tile::*;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tile Editor - v0.0.0".to_string(),
        window_width: 960,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen = render_target(960u32, 600u32);
    screen.texture.set_filter(FilterMode::Nearest);
    let camera = Camera2D {
        zoom: vec2(1.0 / 960.0 * 2.0, 1.0 / 600.0 * 2.0),
        render_target: Some(screen),
        ..Default::default()
    };

    let tilemap = load_tilemap("current.tilemap".to_string()).await;

    loop {
        set_camera(&camera);

        clear_background(BLACK);
        for y in 0..tilemap.tiles[0].len() {
            for x in 0..tilemap.tiles[0][0].len() {
                draw_rectangle_lines(
                    x as f32 * tilemap.tile_size as f32,
                    y as f32 * tilemap.tile_size as f32,
                    tilemap.tile_size as f32 + 1.0, tilemap.tile_size as f32 + 1.0,
                    1.05,
                    LIGHTGRAY,
                );
            }
        }
        tilemap.render(TilemapRenderParams::default());

        set_default_camera();

        draw_texture_ex(
            screen.texture,
            -960.0,
            -600.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(960.0 * 2.0, 600.0 * 2.0)),
                ..Default::default()
            },
        );

        next_frame().await
    }
}