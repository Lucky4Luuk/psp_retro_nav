use std::collections::HashMap;

use osmpbf::{Element, IndexedReader};

mod data;
pub use data::*;

pub fn read_osm_pbf(filename: &str) -> Map {
    let mut reader = IndexedReader::from_path(filename).expect("Failed to open the file!");

    let mut nodes = HashMap::new();
    let mut data_roads = Vec::new();

    let mut coord_min: Option<Coord> = None;
    let mut coord_max: Option<Coord> = None;

    println!("Parsing starting now!");

    let mut road_counter: i32 = 0;

    reader
        .read_ways_and_deps(
            |way| {
                // if way.tags().any(|key_value| {
                //     key_value == ("motorcar", "no") || key_value == ("motor_vehicle", "no")
                // }) {
                //     return false;
                // }
                // way.tags().any(|(k, _v)| k == "highway")
                true
            },
            |element| match element {
                Element::Way(way) => {
                    let tags = way
                        .tags()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect::<HashMap<String, String>>();

                    if let Some(road_kind) = tags.get("highway") {
                        let car_allowed = !(tags.get("motorcar") == Some(&String::from("no"))
                            || tags.get("motor_vehicle") == Some(&String::from("no")));
                        if car_allowed {
                            let id = way.id();
                            let nodes = way.refs().collect();
                            // Default road width = 1 meter
                            let width = tags
                                .get("width")
                                .map(|s| match s {
                                    s if s.ends_with("m") => s[..s.len() - 1].parse::<f32>().ok(),
                                    s if s.ends_with("cm") => {
                                        s[..s.len() - 2].parse::<f32>().map(|n| n * 100.0).ok()
                                    }
                                    _ => s.parse::<f32>().ok(),
                                })
                                .flatten()
                                .unwrap_or(1f32);
                            let speedlimit = tags
                                .get("speedlimit")
                                .map(|s| s.parse::<f32>().ok()) // We parse as f32 in case it says like 100.0 or something lol
                                .flatten()
                                .unwrap_or(0f32) as u8;
                            let data = RoadRaw {
                                id,
                                nodes,
                                kind: road_kind.clone(),
                                width,
                                speedlimit,
                            };
                            data_roads.push(data);

                            road_counter += 1;
                            let blink = 10i32
                                .pow(road_counter.checked_ilog10().unwrap_or(1))
                                .max(100_000);
                            if road_counter % blink == 0 {
                                println!("Parsed {road_counter} roads.");
                            }
                        }
                    }
                }
                Element::Node(node) => {
                    let id = node.id();
                    // let tags = node
                    //     .tags()
                    //     .map(|(key, value)| (key.to_string(), value.to_string()))
                    //     .collect::<HashMap<String, String>>();
                    let coord = Coord {
                        lat: node.lat(),
                        lon: node.lon(),
                    };
                    coord_min = Some(
                        coord_min
                            .map(|other| other.min_each(coord))
                            .unwrap_or(coord),
                    );
                    coord_max = Some(
                        coord_max
                            .map(|other| other.max_each(coord))
                            .unwrap_or(coord),
                    );
                    let data = Node { id, coord };
                    nodes.insert(id, data);
                }
                Element::DenseNode(node) => {
                    let id = node.id();
                    // let tags = node
                    //     .tags()
                    //     .map(|(key, value)| (key.to_string(), value.to_string()))
                    //     .collect::<HashMap<String, String>>();
                    let coord = Coord {
                        lat: node.lat(),
                        lon: node.lon(),
                    };
                    coord_min = Some(
                        coord_min
                            .map(|other| other.min_each(coord))
                            .unwrap_or(coord),
                    );
                    coord_max = Some(
                        coord_max
                            .map(|other| other.max_each(coord))
                            .unwrap_or(coord),
                    );
                    let data = Node { id, coord };
                    nodes.insert(id, data);
                }
                _ => {}
            },
        )
        .unwrap();

    let roads: Vec<Road> = data_roads
        .into_iter()
        .filter_map(|data| {
            let points: Vec<Coord> = data
                .nodes
                .into_iter()
                .filter_map(|i| nodes.get(&i))
                .map(|node| node.coord)
                .collect();
            if points.len() < 2 {
                return None;
            }
            let mut road_min = points[0];
            let mut road_max = points[0];
            for p in &points {
                road_min = road_min.min_each(*p);
                road_max = road_max.max_each(*p);
            }
            Some(Road {
                extent: (road_min, road_max),
                points,
                width: data.width,
                speedlimit: data.speedlimit,
            })
        })
        .collect();

    let objects = Vec::new(); // TODO: Gotta do this still lol

    let coord_min = coord_min.unwrap();
    let coord_max = coord_max.unwrap();

    println!("Min / max: {:?} / {:?}", coord_min, coord_max);

    Map {
        roads,
        objects,
        extent: (coord_min, coord_max),
    }
}
