use tile::*;
use macroquad::prelude::*;

struct Master {
    tilemap: Tilemap,
    camera_pos: Vec2,
    mouse_down_pos: Vec2,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tile Editor - v0.0.0".to_string(),
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

    let mut master = Master {
        tilemap: load_tilemap("current.tilemap".to_string()).await,
        camera_pos: Vec2::ZERO,
        mouse_down_pos: Vec2::ZERO,
    };

    loop {
        update(&mut master);

        camera.target = master.camera_pos.round();
        set_camera(&camera);

        render(&master);

        next_frame().await        
    }
}

fn update(master: &mut Master) {
    if is_key_pressed(KeyCode::Space) {
        master.camera_pos = Vec2::ZERO;
    }
    if is_mouse_button_pressed(MouseButton::Middle) {
        master.mouse_down_pos = master.camera_pos + vec2(480.0, 300.0) + get_mouse_position();
    }
    if is_mouse_button_down(MouseButton::Middle) {
        master.camera_pos = master.mouse_down_pos - vec2(480.0, 300.0) - get_mouse_position();
    }

    if is_mouse_button_down(MouseButton::Left) {
        set_tile_at_mouse(master, 1);
    }

    if is_mouse_button_down(MouseButton::Right) {
        set_tile_at_mouse(master, 0);
    }
}

fn render(master: &Master) {
    clear_background(BLACK);
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
    master.tilemap.render(TilemapRenderParams::default());
}

fn get_mouse_position() -> Vec2 {
    Vec2::from(mouse_position()) * 0.5
}

fn set_tile_at_mouse(master: &mut Master, value: u16) {
    let mouse_pos = get_mouse_position() + master.camera_pos - vec2(240.0, 150.0);
    master.tilemap.tiles[0][(mouse_pos.y / 16.0) as usize][(mouse_pos.x / 16.0) as usize] = value;
}