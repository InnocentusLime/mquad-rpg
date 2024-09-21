mod render;

use macroquad::prelude::*;
use macroquad_tiled::*;
use render::{Render, RenderModel, RenderTile};

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
        &[("castle.png", tileset_tex.clone())],
        &[("castle.tsj", &tileset)],
    ).unwrap();

    info!("{:?}", map);

    let mut render = Render::new();

    let mut player_pos = ivec2(9 * 32, 8 * 32);
    let player_sort_offset = ivec2(0, 32);

    let tileset = &map.tilesets["castle"];
    let tile_tex_rect = |id: u32| {
        let x = (id % tileset.columns) * (tileset.tilewidth as u32);
        let y = (id / tileset.columns) * (tileset.tileheight as u32);

        Rect {
            x: x as f32,
            y: y as f32,
            w: tileset.tilewidth as f32,
            h: tileset.tileheight as f32,
        }
    };
    let layers = ["FloorWall", "DecorObjs"]
        .into_iter()
        .map(|x| &map.layers[x])
        .map(|layer| {
            (0..layer.height).flat_map(|y| {
                (0..layer.width).map(move |x| (x, y))
            })
            .filter_map(|(x, y)| {
                let idx = (x + y * layer.width) as usize;
                Some((
                    (x, y),
                    layer.data[idx].as_ref()?
                ))
            })
            .map(|((x, y), tile)| RenderTile {
                z_order: 0,
                pos: ivec2(x as i32, y as i32) * 32,
                sort_offset: if tile.id == 146 || tile.id == 147 || tile.id == 148 {
                    ivec2(0, -32)
                } else { ivec2(0, 0) },
                tex_rect: tile_tex_rect(tile.id),
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();


    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::A) {
            player_pos.x -= 1;
        }
        if is_key_down(KeyCode::D) {
            player_pos.x += 1;
        }
        if is_key_down(KeyCode::W) {
            player_pos.y -= 1;
        }
        if is_key_down(KeyCode::S) {
            player_pos.y += 1;
        }

        let player_tile = RenderTile {
            z_order: 0,
            pos: player_pos,
            sort_offset: player_sort_offset,
            tex_rect: Rect {
                x: 224.0,
                y: 544.0,
                w: 32.0,
                h: 64.0,
            },
        };

        for (i, layer) in layers.iter().enumerate() {
            if i == 1 {
                render.draw_tiles(
                    layer.iter().map(|x| *x)
                    .chain(std::iter::once(player_tile)),
                    &tileset.texture
                );
            } else {
                render.draw_tiles(
                    layer.iter().map(|x| *x),
                    &tileset.texture
                );
            }
        }

        draw_circle(
            (player_pos.x + player_sort_offset.x) as f32,
            (player_pos.y + player_sort_offset.y) as f32,
            2.0,
            RED
        );

        draw_text(&format!("FPS: {}", get_fps()), 0.0, 20.0, 16.0, RED);

        next_frame().await
    }
}
