// TODO: Use clap or argh to make this a proper CLI tool

use crate::config::*;

pub mod config;
pub mod mapper;
pub mod reader;
pub mod render;

fn main() {
    println!("Hello, world!");

    // let filename = "maps/europe-reduced.osm.pbf";
    let filename = "maps/small.osm.pbf";

    let config = Config {
        mapping: ConfigMapping {
            tile_res: 64,
            zoom: 19,
        },
        style: ConfigStyle {
            road_color: [96; 3],
        },
    };

    let map = reader::read_osm_pbf(filename);
    println!("Roads: {}", map.roads.len());

    let map_tiles = mapper::map_to_tiles(&config, map);
    println!("Tiles: {}", map_tiles.tiles.len());

    render::render_result_to_folder(&config, map_tiles);
}
