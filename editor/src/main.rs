use std::path::Path;
use std::io::Write;
use std::fs::File;
use tile::*;
use macroquad::prelude::*;

struct Master {
    tilemap: Tilemap,
    camera_pos: Vec2,
    mouse_down_pos: Vec2,
    selected_tile: u16,
    selected_layer: usize,
    show_grid: bool,
}

impl Default for Master {
    fn default() -> Self {
        Self {
            tilemap: Tilemap::default(),
            camera_pos: Vec2::ZERO,
            mouse_down_pos: Vec2::ZERO,
            selected_tile: 1,
            selected_layer: 0,
            show_grid: true,
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tile Editor - v0.2.0".to_string(),
        window_width: 960,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: vec2(1.0 / 480.0 * 2.0, -(1.0 / 300.0 * 2.0)),
        ..Default::default()
    };

    let mut master = Master::default();

    if !Path::new("current.tilemap").exists() {
        save(&master);
    }

    master.tilemap = load_tilemap("current.tilemap".to_string()).await;
    master.tilemap.texture.set_filter(FilterMode::Nearest);

    loop {
        update(&mut master);

        camera.target = master.camera_pos.round();
        set_camera(&camera);

        render(&master);

        next_frame().await        
    }
}

fn update(master: &mut Master) {
    if is_mouse_button_pressed(MouseButton::Middle) {
        master.mouse_down_pos = master.camera_pos + vec2(480.0, 300.0) + get_mouse_position();
    }
    if is_mouse_button_down(MouseButton::Middle) {
        master.camera_pos = master.mouse_down_pos - vec2(480.0, 300.0) - get_mouse_position();
    }

    if is_key_pressed(KeyCode::A) {
        if master.selected_layer == 0 {
            master.selected_layer = master.tilemap.tiles.len() - 1;
        } else {
            master.selected_layer -= 1;
        }
    }
    if is_key_pressed(KeyCode::S) {
        if is_key_down(KeyCode::LeftShift) {
            save(master);
        } else {
            if master.selected_layer == master.tilemap.tiles.len() - 1 {
                master.selected_layer = 0;
            } else {
                master.selected_layer += 1;
            }
        }
    }

    if is_key_pressed(KeyCode::Q) {
        master.selected_tile -= 1;
        if master.selected_tile < 1 {
            master.selected_tile = (master.tilemap.texture.width() / master.tilemap.tile_size as f32).round() as u16;
        }
    }
    if is_key_pressed(KeyCode::W) {
        master.selected_tile += 1;
        if master.selected_tile > (master.tilemap.texture.width() / master.tilemap.tile_size as f32).round() as u16 {
            master.selected_tile = 1;
        }
    }

    if is_mouse_button_down(MouseButton::Left) {
        set_tile_at_mouse(master, master.selected_tile);
    }

    if is_mouse_button_down(MouseButton::Right) {
        set_tile_at_mouse(master, 0);
    }

    if is_key_pressed(KeyCode::G) {
        master.show_grid = !master.show_grid;
    }
}

fn render(master: &Master) {
    clear_background(BLACK);
    if master.show_grid {
        for y in 0..master.tilemap.tiles[0].len() {
            for x in 0..master.tilemap.tiles[0][0].len() {
                draw_rectangle_lines(
                    x as f32 * master.tilemap.tile_size as f32,
                    y as f32 * master.tilemap.tile_size as f32,
                    master.tilemap.tile_size as f32 + 1.0, master.tilemap.tile_size as f32 + 1.0,
                    1.05,
                    LIGHTGRAY,
                );
            }
        }
    }
    // I'm doing custom rendering so I can gray out other layers
    // master.tilemap.render(TilemapRenderParams::default());
    for z in 0..master.tilemap.tiles.len() {
        for y in 0..master.tilemap.tiles[z].len() {
            for x in 0..master.tilemap.tiles[z][y].len() {
                if master.tilemap.tiles[z][y][x] != 0 {
                    draw_texture_ex(
                        master.tilemap.texture,
                        (x as f32 * master.tilemap.tile_size as f32).round(),
                        (y as f32 * master.tilemap.tile_size as f32).round(),
                        if z == master.selected_layer {
                            WHITE
                        } else {
                            Color {
                                r: 0.9,
                                g: 0.9,
                                b: 0.9,
                                a: 0.5,
                            }
                        },
                        DrawTextureParams {
                            source: Some(Rect {
                                x: ((master.tilemap.tiles[z][y][x] - 1) * master.tilemap.tile_size) as f32,
                                y: 0.0,
                                w: master.tilemap.tile_size as f32,
                                h: master.tilemap.tile_size as f32,
                            }),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }

    let mouse_pos = (get_mouse_position() + master.camera_pos - vec2(240.0, 150.0)) / (master.tilemap.tile_size as f32) - vec2(0.5, 0.5);
    draw_texture_ex(
        master.tilemap.texture,
        clamp(mouse_pos.x.round(), 0.0, (master.tilemap.tiles[master.selected_layer][0].len() - 1) as f32) * (master.tilemap.tile_size as f32),
        clamp(mouse_pos.y.round(), 0.0, (master.tilemap.tiles[master.selected_layer].len() - 1) as f32) * (master.tilemap.tile_size as f32),
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.5,
        },
        DrawTextureParams {
            source: Some(Rect {
                x: ((master.selected_tile - 1) * master.tilemap.tile_size) as f32,
                y: 0.0,
                w: master.tilemap.tile_size as f32,
                h: master.tilemap.tile_size as f32,
            }),
            ..Default::default()
        },
    );
}

fn get_mouse_position() -> Vec2 {
    Vec2::from(mouse_position()) * 0.5
}

fn set_tile_at_mouse(master: &mut Master, value: u16) {
    let mut mouse_pos = (get_mouse_position() + master.camera_pos - vec2(240.0, 150.0)) / (master.tilemap.tile_size as f32) - vec2(0.5, 0.5);
    mouse_pos = vec2(
        clamp(mouse_pos.x.round(), 0.0, (master.tilemap.tiles[master.selected_layer][0].len() - 1) as f32),
        clamp(mouse_pos.y.round(), 0.0, (master.tilemap.tiles[master.selected_layer].len() - 1) as f32),
    );
    master.tilemap.tiles[master.selected_layer][mouse_pos.y as usize][mouse_pos.x as usize] = value;
}

fn save(master: &Master) {
    let mut save_file = File::create(get_file_path("current.tilemap".to_string())).unwrap();
    let mut save = "\"".to_string();
    save += &format!("{}\"", master.tilemap.tile_size);
    write!(save_file, "{}", save).unwrap();
}