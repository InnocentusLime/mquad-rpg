mod render;

use macroquad::prelude::*;
use macroquad_tiled::*;
use render::{Render, RenderModel};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    TitleScreen,
    LoadingSave,
    UiInteraction,
}

#[macroquad::main("Silli RPG")]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);

    let map = load_string("./assets/castle_test.tmj").await.unwrap();
    let tileset = load_string("./assets/castle.tsj").await.unwrap();
    let tileset_tex = load_texture("./assets/castle.png").await.unwrap();
    let map = load_map(
        &map,
        &[("castle.png", tileset_tex)],
        &[("castle.tsj", &tileset)],
    ).unwrap();

    let mut render = Render::new();

    let mut pos = ivec2(9 * 32, 8 * 32);

    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::A) {
            pos.x -= 1;
        }
        if is_key_down(KeyCode::D) {
            pos.x += 1;
        }
        if is_key_down(KeyCode::W) {
            pos.y -= 1;
        }
        if is_key_down(KeyCode::S) {
            pos.y += 1;
        }

        draw_text("Hey stinky", 0.0, 20.0, 16.0, RED);

        render.draw(&RenderModel {
            map: Some(&map),
            actors: &[
                (pos, 2, RED, uvec2(16, 64)),
            ],
        });

        next_frame().await
    }
}
