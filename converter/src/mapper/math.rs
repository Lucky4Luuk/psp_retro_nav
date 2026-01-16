//! This little "library" implements mapping latitude/longitude degrees directly
//! to tiles using a "zoom level" and a tile resolution
//! Implemented according to https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames
//! See https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Zoom_levels for more information on zoom levels

use core::f64::consts::PI;

use super::GlobalLocation;
use crate::reader::Coord;

/// Projects coordinates using web mercator projection
fn to_web_mercator(coord: Coord) -> (f64, f64) {
    (coord.lon, (coord.lat * PI / 180.0).tan().asinh())
}

/// Returns (tile_x, tile_y, pixel_x, pixel_y)
/// Zoom level corresponds to https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Zoom_levels
pub fn coord_to_tile(coord: Coord, zoom: u8, tile_res: u32) -> GlobalLocation {
    let (mut x, mut y) = to_web_mercator(coord);

    // Map projected coordinates onto a unit square
    x = 0.5 + x / 360.0;
    y = 0.5 - y / (2.0 * PI);

    // Calculate location in tile space
    let n = 2u32.pow(zoom as u32) as f64;
    x *= n;
    y *= n;

    // Get local tile space coordinate and transform into pixel space
    // and just return immediately
    GlobalLocation {
        tile_x: x.floor() as u32,
        tile_y: y.floor() as u32,
        x: (x.fract() * tile_res as f64) as u32,
        y: (y.fract() * tile_res as f64) as u32,
    }
}
