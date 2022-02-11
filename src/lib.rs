use macroquad::file::load_string;

pub fn load_tilemap(path: String) -> Vec<Vec<Vec<u16>>> {
    let tilemap: Vec<Vec<Vec<u16>>> = Vec::new();

    let file = load_string(&path);

    tilemap
}