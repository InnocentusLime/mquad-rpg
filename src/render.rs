use macroquad::prelude::*;
use macroquad_tiled::*;

pub struct RenderModel<'a> {
    pub map: Option<&'a Map>,
    pub actors: &'a [(IVec2, u32, Color)],
}

pub struct Render {
    layer_buff: Vec<(u32, String)>,
    actor_buff: Vec<(IVec2, u32, Color)>,
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
            self.draw_map(map);
        }
    }

    fn draw_map(&mut self, map: &Map) {
        self.layer_buff.clear();
        self.actor_buff.clear();

        self.layer_buff.extend(map.raw_tiled_map.layers.iter()
            .map(|layer| {
                let idx = layer.properties.iter().find(|x| x.name == "layer_order")
                            .map(|x| x.value.parse::<u32>().unwrap())
                            .unwrap_or(0);
                (idx, layer.name.clone())
            })
        );

        self.layer_buff.sort_by_key(|&(idx, _)| idx);
        self.actor_buff.sort_by_key(|&(pos, layer, _)| (pos.y, layer));
    }
}