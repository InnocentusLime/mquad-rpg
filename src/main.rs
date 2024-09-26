mod render;
mod urect;

use macroquad::prelude::*;
use macroquad_tiled::*;
use render::{Render, RenderLayer, RenderModel, RenderTile};
use urect::URect;

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
    let raw_tileset = map.raw_tiled_map.tilesets.iter()
        .find(|x| x.name == "castle")
        .unwrap();
    let get_tile_sort_off = |tile: &Tile| {
        assert_eq!(tile.attrs, "Tile");
        let props = &raw_tileset.tiles[tile.id as usize].properties;
        let sort_off_x = props.iter().find(|x| x.name == "sort_off_x")
            .map(|x| match x.value {
                PropertyVal::Integer(x) => x as i32,
                PropertyVal::UInt(x) => x as i32,
                _ => panic!("Wrong sort_off_x type"),
            })
            .unwrap_or(0);
        let sort_off_y = props.iter().find(|x| x.name == "sort_off_y")
            .map(|x| match x.value {
                PropertyVal::Integer(x) => x as i32,
                PropertyVal::UInt(x) => x as i32,
                _ => panic!("Wrong sort_off_y type"),
            })
            .unwrap_or(0);

        ivec2(sort_off_x, sort_off_y)
    };

    let mut actor_layer = None;
    let mut layer_groups = vec![Vec::<RenderLayer>::new()];

    let layer_it = map.raw_tiled_map.layers.iter()
        .enumerate();

    for (i, layer) in layer_it {
        if layer.name == "Actors" {
            actor_layer = Some(i);
            continue;
        }
        // match layer.ty.as_str() {
        //     "ActorLayer" => {
        //         actor_layer = Some(i);
        //         continue;
        //     },
        //     "RenderLayer" => (),
        //     x => panic!("Unknown layer type: {x:?}"),
        // }

        let same_group = layer.properties.iter()
            .find(|x| x.name == "merge_with_prev")
            .map(|x| match x.value {
                PropertyVal::Boolean(x) => x,
                _ => panic!("Wront merge_with_prev type"),
            })
            .unwrap_or(false);

        if !same_group {
            layer_groups.push(Vec::new());
        }

        let z_order = i as u8;
        let unraw = map.layers.get(&layer.name)
            .unwrap();
        layer_groups.last_mut().unwrap().push(RenderLayer {
            name: layer.name.clone(),
            width: layer.width,
            height: layer.height,
            tiles: unraw.data.iter().enumerate().map(|(idx, x)| {
                let idx = idx as u32;
                let tile = x.as_ref()?;
                Some(RenderTile {
                    z_order,
                    pos: ivec2(
                        (idx % layer.width) as i32 * tileset.tilewidth,
                        (idx / layer.width) as i32 * tileset.tileheight
                    ),
                    sort_offset: get_tile_sort_off(tile),
                    tex_rect: tile_tex_rect(tile.id),
                })
            })
            .collect(),
        });
    }

    let mut model = RenderModel {
        layer_groups,
        actors: vec![RenderTile {
            z_order: 0,
            pos: player_pos,
            sort_offset: player_sort_offset,
            tex_rect: Rect {
                x: 224.0,
                y: 544.0,
                w: 32.0,
                h: 64.0,
            },
        }],
        actor_layer: actor_layer.unwrap(),
        atlas: tileset_tex,
    };

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

        model.actors[0] = RenderTile {
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

        render.draw(&model, URect {
            x: 0,
            y: 0,
            w: 30,
            h: 20,
        });

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
