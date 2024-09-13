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

    let mut pos = ivec2(9 * 32, 8 * 32);

    let render_off = ivec2(0, 32);

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


        let floor_layer = &map.layers["FloorWall"];
        let objs_layer = &map.layers["DecorObjs"];
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
        let pos_iter = (0..floor_layer.height).flat_map(|y| {
            (0..floor_layer.width).map(move |x| (x, y))
        });
        render.draw_tiles(
            pos_iter
                .filter_map(|(x, y)| {
                    let idx = (x + y * floor_layer.width) as usize;
                    Some((
                        (x, y),
                        floor_layer.data[idx].as_ref()?
                    ))
                })
                .map(|((x, y), tile)| RenderTile {
                    z_order: 0,
                    pos: ivec2(x as i32, y as i32) * 32,
                    render_off: ivec2(0, 0),
                    tex_rect: tile_tex_rect(tile.id),
                }),
            32.0,
            &tileset_tex
        );
        let pos_iter = (0..objs_layer.height).flat_map(|y| {
            (0..objs_layer.width).map(move |x| (x, y))
        });
        render.draw_tiles(
            pos_iter
                .filter_map(|(x, y)| {
                    let idx = (x + y * objs_layer.width) as usize;
                    Some((
                        (x, y),
                        objs_layer.data[idx].as_ref()?
                    ))
                })
                .map(|((x, y), tile)| RenderTile {
                    z_order: 0,
                    pos: ivec2(x as i32, y as i32) * 32,
                    render_off: if tile.id == 146 || tile.id == 147 || tile.id == 148 {
                        ivec2(0, -32)
                    } else { ivec2(0, 0) },
                    tex_rect: tile_tex_rect(tile.id),
                })
                .chain(std::iter::once(RenderTile {
                    z_order: 0,
                    pos,
                    render_off,
                    tex_rect: Rect {
                        x: 224.0,
                        y: 544.0,
                        w: 32.0,
                        h: 64.0,
                    },
                })),
            32.0,
            &tileset_tex
        );

        draw_circle(
            (pos.x + render_off.x) as f32,
            (pos.y + render_off.y) as f32,
            2.0,
            RED
        );

        // render.draw(&RenderModel {
        //     map: Some(&map),
        //     actors: &[
        //         (pos, 2, RED, uvec2(16, 64)),
        //     ],
        // });
        draw_text(&format!("FPS: {}", get_fps()), 0.0, 20.0, 16.0, RED);

        next_frame().await
    }
}
