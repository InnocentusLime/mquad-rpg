use macroquad::prelude::*;
use macroquad_tiled::*;

const TILE_SIZE: u32 = 32;
pub struct RenderModel<'a> {
    pub map: Option<&'a Map>,
    pub actors: &'a [(IVec2, u32, Color, UVec2)],
}

pub struct Render {
    layer_buff: Vec<(u32, String)>,
    actor_buff: Vec<(IVec2, u32, Color, UVec2)>,
}

impl Render {
    pub fn new() -> Self {
        Self {
            layer_buff: Vec::new(),
            actor_buff: Vec::new(),
        }
    }

    pub fn draw(&mut self, model: &RenderModel) {
        if let Some(map) = model.map {
            self.draw_map(map, model.actors);
        }
    }

    fn draw_map(&mut self, map: &Map, actors: &[(IVec2, u32, Color, UVec2)]) {
        self.layer_buff.clear();
        self.actor_buff.clear();

        self.actor_buff.extend(
            actors.iter().map(|x| *x)
        );
        self.layer_buff.extend(map.raw_tiled_map.layers.iter()
            .map(|layer| {
                let idx = layer.properties.iter().find(|x| x.name == "layer_order")
                            .map(|x| x.value.parse::<u32>().unwrap())
                            .unwrap_or(0);
                (idx, layer.name.clone())
            })
        );

        self.layer_buff.sort_by_key(|&(idx, _)| idx);
        self.actor_buff.sort_by_key(|&(pos, layer, _, size)| (pos.y as u32 + size.y, layer));

        let mut act_curr = 0;
        let width = map.raw_tiled_map.width;
        let height = map.raw_tiled_map.height;
        let pos_iter = (0..height).flat_map(|y| {
            (0..width).map(move |x| (x, y))
        });

        for (layer_idx, layer_id) in &self.layer_buff {
            for (x, y) in pos_iter.clone() {
                let layer = &map.layers[layer_id];

                if let Some(&(pos, layer, col, size)) = self.actor_buff.get(act_curr) {
                    let mut not_draw = layer != *layer_idx;
                    not_draw = not_draw || pos.y < 0;
                    not_draw = not_draw || ((pos.y + size.y as i32) / map.raw_tiled_map.tileheight as i32) != y as i32;

                    if !not_draw {
                        // info!("({}, {}) -- ({}, {})",
                        //     x, y,
                        //     pos.x / map.raw_tiled_map.tilewidth as i32,
                        //     pos.y / map.raw_tiled_map.tileheight as i32,
                        // );

                        draw_rectangle(pos.x as f32, pos.y as f32, size.x as f32, size.y as f32, col);

                        act_curr += 1;
                    }
                }

                if x < layer.width && y < layer.height {
                    let tile = &layer.data[(x + y * layer.width) as usize];
                    if let Some(tile) = tile {
                        let tileset = &map.tilesets[&tile.tileset];
                        let tx = (tile.id % tileset.columns) *
                            (tileset.tilewidth as u32 + tileset.spacing as u32);
                        let ty = tile.id / tileset.columns *
                            (tileset.tileheight as u32 + tileset.spacing as u32);
                        let tile_width = tileset.tilewidth as f32;
                        let tile_height = tileset.tileheight as f32;

                        draw_texture_ex(
                            &tileset.texture,
                            x as f32 * tile_width,
                            y as f32 * tile_height,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(tile_width, tile_height)),
                                source: Some(Rect {
                                    x: tx as f32,
                                    y: ty as f32,
                                    w: tile_width,
                                    h: tile_height,
                                }),
                                ..Default::default()
                            }
                        );
                    }
                }

            }
        }
    }
}