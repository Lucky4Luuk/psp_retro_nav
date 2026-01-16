use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct GlobalLocation {
    pub tile_x: u32,
    pub tile_y: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
pub struct Road {
    pub points: Vec<GlobalLocation>,
    pub width: f32,
    pub speedlimit: u8,
}

pub struct Tile {
    pub location: (u32, u32),
    pub road_indices: Vec<usize>,
    // pub objects: Vec<Object>,
}

impl Tile {
    pub fn empty(location: (u32, u32)) -> Self {
        Self {
            location,
            road_indices: Vec::new(),
        }
    }
}

pub struct MapTiles {
    pub zoom: u8,
    pub tiles: HashMap<u64, Tile>,

    pub roads: Vec<Road>,
}
