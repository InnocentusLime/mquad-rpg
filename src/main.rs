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

    loop {
        clear_background(LIGHTGRAY);

        draw_text("Hey stinky", 0.0, 20.0, 16.0, RED);

        map.draw_tiles("FloorWall", Rect::new(0.0, 0.0, 32.0 * 30.0, 32.0 * 20.0), None);
        map.draw_tiles("Decor", Rect::new(0.0, 0.0, 32.0 * 30.0, 32.0 * 20.0), None);
        map.draw_tiles("DecorObjs", Rect::new(0.0, 0.0, 32.0 * 30.0, 32.0 * 20.0), None);

        render.draw(&RenderModel {
            map: Some(&map),
            actors: &[],
        });

        next_frame().await
    }
}
