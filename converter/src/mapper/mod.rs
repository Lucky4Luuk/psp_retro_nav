//! This transforms our coordinates into the right space for rendering!

use std::collections::HashMap;

use crate::{
    config::Config,
    reader::{Map, Road as ReaderRoad},
};

mod data;
pub use data::*;

mod math;
pub use math::*;

pub fn map_to_tiles(config: &Config, map: Map) -> MapTiles {
    let max_tiles_x = 2u64.pow(config.mapping.zoom as u32);

    let mut mapped_roads = Vec::new();
    let mut tiles = HashMap::new();

    for road in map.roads {
        map_road_to_tiles(
            config.mapping.zoom,
            config.mapping.tile_res,
            max_tiles_x,
            &mut tiles,
            &mut mapped_roads,
            road,
        );
    }

    MapTiles {
        zoom: config.mapping.zoom,
        tiles,

        roads: mapped_roads,
    }
}

fn map_road_to_tiles(
    zoom: u8,
    tile_res: u32,
    max_tiles_x: u64,
    tiles: &mut HashMap<u64, Tile>,
    mapped_roads: &mut Vec<Road>,
    road: ReaderRoad,
) {
    // First we map all the points into tile space
    let mapped_points = road
        .points
        .into_iter()
        .map(|p| coord_to_tile(p, zoom, tile_res))
        .collect::<Vec<_>>();

    // And add the result to our list of mapped roads
    let road_id = mapped_roads.len();
    mapped_roads.push(Road {
        points: mapped_points,
        width: road.width,
        speedlimit: road.speedlimit,
    });

    // Now we need to add the road index to each of the potentially
    // affected tiles (based on the road bbox)
    let pos_min = coord_to_tile(road.extent.0, zoom, tile_res);
    let pos_max = coord_to_tile(road.extent.1, zoom, tile_res);

    // panic!("pos_min: {:?}\npos_max: {:?}", pos_min, pos_max);

    for tx in pos_min.tile_x..pos_max.tile_x {
        for ty in pos_max.tile_y..pos_min.tile_y {
            let tile_id = tx as u64 + max_tiles_x * ty as u64;
            if !tiles.contains_key(&tile_id) {
                let new_tile = Tile::empty((tx, ty));
                tiles.insert(tile_id, new_tile);
            }

            let tile_ref = tiles.get_mut(&tile_id).unwrap();
            tile_ref.road_indices.push(road_id);
        }
    }
}
