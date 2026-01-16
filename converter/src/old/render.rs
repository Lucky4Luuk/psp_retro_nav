use glam::Vec2;
use imageproc::point::Point;

use crate::data::*;

pub struct StyleConfig {
    /// Multiplier on road thickness (width is based on actual map data)
    pub road_thickness_mult: f32,

    // Colors
    pub road_color: [u8; 3],
}

pub struct RenderConfig {
    /// In pixels
    pub tile_res: usize,
    /// In meters
    pub tile_size: usize,

    pub style: StyleConfig,
}

struct RenderRoad {
    points: Vec<Vec2>,
    width: f32,
}

struct RenderMap {
    roads: Vec<RenderRoad>,
}

pub fn render_to_tiles(config: RenderConfig, mut map: Map) {
    let (map_min_x, map_max_y) = lat_lon_to_mercator_meters(map.extent.1.lat, map.extent.0.lon);
    let (map_max_x, map_min_y) = lat_lon_to_mercator_meters(map.extent.0.lat, map.extent.1.lon);

    let map_width_m = map_max_x - map_min_x;
    let map_height_m = map_max_y - map_min_y;

    let tile_px = config.tile_res as f64;

    let pixels_per_meter = tile_px / (config.tile_size as f64);

    let map_width_px = map_width_m * pixels_per_meter;
    let map_height_px = map_height_m * pixels_per_meter;

    let tile_count_x = (map_width_px / tile_px).ceil();
    let tile_count_y = (map_height_px / tile_px).ceil();

    let image_width_px = tile_count_x * tile_px;
    let image_height_px = tile_count_y * tile_px;

    let offset_x_px = (image_width_px - map_width_px) * 0.5;
    let offset_y_px = (image_height_px - map_height_px) * 0.5;

    println!("Transforming map data to image space...");
    let render_map = transform_map_space(&config, offset_x_px, offset_y_px, map);

    println!("Rendering tiles!");
    render_to_tiles_impl(
        &config,
        &render_map,
        tile_count_x as usize,
        tile_count_y as usize,
    );
}

fn render_to_tiles_impl(
    config: &RenderConfig,
    map: &RenderMap,
    tile_count_x: usize,
    tile_count_y: usize,
) {
    let full_img_res = (
        tile_count_x * config.tile_res,
        tile_count_y * config.tile_res,
    );

    println!("Image res: {:?}", full_img_res);

    let mut img = image::RgbImage::new(full_img_res.0 as u32, full_img_res.1 as u32);

    for road in &map.roads {
        draw_road(&mut img, config, road);
    }

    img.save("tmp.png").unwrap();
}

fn draw_road(img: &mut image::RgbImage, config: &RenderConfig, road: &RenderRoad) {
    let pixels_per_meter = config.tile_size / config.tile_res;

    let width = road.width * config.style.road_thickness_mult * (pixels_per_meter as f32);
    let polygon = thick_polyline_polygon(&road.points, width)
        .into_iter()
        .map(|v| Point {
            x: v.x as i32,
            y: v.y as i32,
        })
        .collect::<Vec<Point<i32>>>();

    imageproc::drawing::draw_antialiased_polygon_mut(
        img,
        &polygon,
        image::Rgb(config.style.road_color),
        |a, b, f| imageproc::pixelops::interpolate(a, b, f),
    );

    for i in 0..(road.points.len() - 1) {
        let start = road.points[i];
        let end = road.points[i + 1];
        imageproc::drawing::draw_antialiased_line_segment_mut(
            img,
            (start.x as i32, start.y as i32),
            (end.x as i32, end.y as i32),
            image::Rgb([255, 0, 0]),
            |a, b, f| imageproc::pixelops::interpolate(a, b, f),
        );
    }
}

/// Generate a thick polyline polygon
pub fn thick_polyline_polygon(points: &[Vec2], thickness: f32) -> Vec<Vec2> {
    if points.len() < 2 {
        return vec![];
    }

    let half = thickness * 0.5;
    let perp = |v: Vec2| Vec2::new(-v.y, v.x);

    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    for i in 0..points.len() {
        let p = points[i];

        let offset = if i == 0 {
            // First segment
            perp((points[1] - p).normalize()) * half
        } else if i == points.len() - 1 {
            // Last segment
            perp((p - points[i - 1]).normalize()) * half
        } else {
            // Middle segment, simple average of previous and next normals
            let dir_prev = (p - points[i - 1]).normalize();
            let dir_next = (points[i + 1] - p).normalize();
            let n_prev = perp(dir_prev);
            let n_next = perp(dir_next);
            (n_prev + n_next).normalize() * half
        };

        left_side.push(p + offset);
        right_side.push(p - offset);
    }

    // Combine left side and reversed right side to form a closed polygon
    left_side.extend(right_side.into_iter().rev());
    left_side
}

const EARTH_RADIUS: f64 = 6_378_137.0;
const MAX_LAT: f64 = 85.05112878;

fn lat_lon_to_mercator_meters(lat: f64, lon: f64) -> (f64, f64) {
    let lat = lat.clamp(-MAX_LAT, MAX_LAT);

    let x = lon.to_radians() * EARTH_RADIUS;
    let y = (lat.to_radians().tan().ln()) * EARTH_RADIUS;

    (x, y)
}

pub fn lat_lon_to_pixel(
    lat: f64,
    lon: f64,

    // Map bounds (precise)
    _lat_min: f64,
    lat_max: f64,
    lon_min: f64,
    _lon_max: f64,

    // Tile parameters
    tile_res_px: f64,
    tile_size_m: f64,

    // Precomputed offsets
    offset_x_px: f64,
    offset_y_px: f64,
) -> (f64, f64) {
    let pixels_per_meter = tile_res_px / tile_size_m;

    // Project point
    let (mx, my) = lat_lon_to_mercator_meters(lat, lon);

    // Project map origin (top-left of map, NOT image)
    let (map_min_x, map_max_y) = lat_lon_to_mercator_meters(lat_max, lon_min);

    // Relative to map
    let dx_m = mx - map_min_x;
    let dy_m = map_max_y - my;

    // Map meters → pixels, then offset into image
    let px = dx_m * pixels_per_meter + offset_x_px;
    let py = dy_m * pixels_per_meter + offset_y_px;

    (px, py)
}

fn transform_map_space(
    config: &RenderConfig,
    offset_x_px: f64,
    offset_y_px: f64,
    map: Map,
) -> RenderMap {
    let roads = map
        .roads
        .into_iter()
        .map(|road| {
            let points = road
                .points
                .into_iter()
                .map(|coord| {
                    let (x, y) = lat_lon_to_pixel(
                        coord.lat,
                        coord.lon,
                        map.extent.0.lat,
                        map.extent.1.lat,
                        map.extent.0.lon,
                        map.extent.1.lon,
                        config.tile_res as f64,
                        config.tile_size as f64,
                        offset_x_px,
                        offset_y_px,
                    );
                    Vec2::new(x as f32, y as f32)
                })
                .collect::<Vec<_>>();
            RenderRoad {
                points,
                width: road.width,
            }
        })
        .collect::<Vec<_>>();

    RenderMap { roads }
}
