use macroquad::prelude::*;
use macroquad_tiled::*;

use crate::urect::URect;

const TILE_SIZE: u32 = 32;

#[derive(Debug, Clone, Copy)]
pub struct RenderTile {
    pub z_order: u8,
    pub pos: IVec2,
    pub sort_offset: IVec2,
    pub tex_rect: Rect,
}

pub struct RenderLayer {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Option<RenderTile>>,
}

impl RenderLayer {
    fn iter_tiles(&'_ self, cover_rect: URect) -> impl Iterator<Item = RenderTile> + '_ {
        let pos_iter = (cover_rect.top()..cover_rect.bottom())
            .flat_map(move |x| (cover_rect.left()..cover_rect.right())
                .map(move |y| (x, y)));

        pos_iter
            .filter(|&(x, y)| x < self.width && y < self.height)
            .map(|(x, y)| x + y * self.width)
            .filter_map(|idx| *self.tiles.get(idx as usize)?)
    }
}

pub struct RenderModel {
    pub layer_groups: Vec<Vec<RenderLayer>>,
    pub actors: Vec<RenderTile>,
    pub actor_layer: usize,
    pub atlas: Texture2D,
}

impl RenderModel {
    // fn t(&self, u: URect) {
    //     for x in &self.layer_groups {
    //         let x = x.iter().flat_map(|x| x.iter_tiles(u));
    //     }
    // }
}

pub struct Render {
    tile_buffer: Vec<RenderTile>,
}

impl Render {
    pub fn new() -> Self {
        Self {
            tile_buffer: vec![],
        }
    }

    pub fn draw(
        &mut self,
        model: &RenderModel,
        view_rect: URect,
    ) {
        let mut layer_idx = 0;
        for group in &model.layer_groups {
            for layer in group {
                if layer_idx == model.actor_layer {
                    self.buffer_tiles(model.actors.iter().map(|x| *x));
                    layer_idx += 1;
                }

                self.buffer_tiles(layer.iter_tiles(view_rect));
                layer_idx += 1;
            }
            self.draw_tiles(&model.atlas);
        }
    }

    pub fn buffer_tiles(
        &mut self,
        tiles: impl Iterator<Item = RenderTile>,
    ) {
        self.tile_buffer.extend(tiles.map(|tile| RenderTile {
            pos: (tile.pos + tile.sort_offset),
            sort_offset: -tile.sort_offset,
            ..tile
        }));
    }

    pub fn draw_tiles(
        &mut self,
        atlas: &Texture2D,
    ) {
        self.tile_buffer.sort_by_key(|tile| {
            (tile.pos.y, tile.pos.x, tile.z_order)
        });

        self.tile_buffer.iter().for_each(|tile| {
            let pos = (tile.pos + tile.sort_offset).as_vec2();
            draw_texture_ex(
                atlas,
                pos.x,
                pos.y,
                WHITE,
                DrawTextureParams {
                    // dest_size: Some(vec2(tile_size, tile_size)),
                    source: Some(tile.tex_rect),
                    ..Default::default()
                });
        });

        self.tile_buffer.clear();
    }

    // fn draw_map(&mut self, map: &Map, actors: &[(IVec2, u32, Color, UVec2)]) {
    //     self.layer_buff.clear();
    //     self.actor_buff.clear();

    //     self.actor_buff.extend(
    //         actors.iter().map(|x| *x)
    //     );
    //     self.layer_buff.extend(map.raw_tiled_map.layers.iter()
    //         .map(|layer| {
    //             let idx = layer.properties.iter().find(|x| x.name == "layer_order")
    //                         .map(|x| match &x.value {
    //                             PropertyVal::UInt(x) => *x as u32,
    //                             _ => panic!("Ouch")
    //                         })
    //                         .unwrap_or(0);
    //             (idx, layer.name.clone())
    //         })
    //     );

    //     self.layer_buff.sort_by_key(|&(idx, _)| idx);
    //     self.actor_buff.sort_by_key(|&(pos, layer, _, size)| (pos.y as u32 + size.y, layer));

    //     let mut act_curr = 0;
    //     let width = map.raw_tiled_map.width;
    //     let height = map.raw_tiled_map.height;
    //     let pos_iter = (0..height).flat_map(|y| {
    //         (0..width).map(move |x| (x, y))
    //     });

    //     for (layer_idx, layer_id) in &self.layer_buff {
    //         for (x, y) in pos_iter.clone() {
    //             let layer = &map.layers[layer_id];

    //             if let Some(&(pos, layer, col, size)) = self.actor_buff.get(act_curr) {
    //                 let mut not_draw = layer != *layer_idx;
    //                 not_draw = not_draw || pos.y < 0;
    //                 not_draw = not_draw || ((pos.y + size.y as i32) / map.raw_tiled_map.tileheight as i32) != y as i32;

    //                 if !not_draw {
    //                     // info!("({}, {}) -- ({}, {})",
    //                     //     x, y,
    //                     //     pos.x / map.raw_tiled_map.tilewidth as i32,
    //                     //     pos.y / map.raw_tiled_map.tileheight as i32,
    //                     // );

    //                     draw_rectangle(pos.x as f32, pos.y as f32, size.x as f32, size.y as f32, col);

    //                     act_curr += 1;
    //                 }
    //             }

    //             if x < layer.width && y < layer.height {
    //                 let tile = &layer.data[(x + y * layer.width) as usize];
    //                 if let Some(tile) = tile {
    //                     let tileset = &map.tilesets[&tile.tileset];
    //                     let tx = (tile.id % tileset.columns) *
    //                         (tileset.tilewidth as u32 + tileset.spacing as u32);
    //                     let ty = tile.id / tileset.columns *
    //                         (tileset.tileheight as u32 + tileset.spacing as u32);
    //                     let tile_width = tileset.tilewidth as f32;
    //                     let tile_height = tileset.tileheight as f32;

    //                     draw_texture_ex(
    //                         &tileset.texture,
    //                         x as f32 * tile_width,
    //                         y as f32 * tile_height,
    //                         WHITE,
    //                         DrawTextureParams {
    //                             dest_size: Some(vec2(tile_width, tile_height)),
    //                             source: Some(Rect {
    //                                 x: tx as f32,
    //                                 y: ty as f32,
    //                                 w: tile_width,
    //                                 h: tile_height,
    //                             }),
    //                             ..Default::default()
    //                         }
    //                     );
    //                 }
    //             }

    //         }
    //     }
    // }
}