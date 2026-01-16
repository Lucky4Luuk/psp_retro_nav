use crate::Config;
use crate::mapper::*;

pub fn render_result_to_folder(config: &Config, map_tiles: MapTiles) {
    let mut tile_x_min = u32::MAX;
    let mut tile_y_min = u32::MAX;
    let mut tile_x_max = 0;
    let mut tile_y_max = 0;

    for (_id, tile) in &map_tiles.tiles {
        tile_x_min = tile_x_min.min(tile.location.0);
        tile_y_min = tile_y_min.min(tile.location.1);
        tile_x_max = tile_x_max.max(tile.location.0);
        tile_y_max = tile_y_max.max(tile.location.1);
    }

    // temporarily render out a basic test image
    let img_width = (tile_x_max - tile_x_min + 2) * (config.mapping.tile_res as u32);
    let img_height = (tile_y_max - tile_y_min + 2) * (config.mapping.tile_res as u32);
    println!("Rendering image with size ({img_width} x {img_height})");
    let mut img = image::RgbImage::new(img_width, img_height);

    for road in map_tiles.roads {
        draw_road(config, &mut img, tile_x_min, tile_y_min, road);
    }

    img.save("tmp.png").unwrap();
}

fn draw_road(config: &Config, img: &mut image::RgbImage, offset_x: u32, offset_y: u32, road: Road) {
    for i in 0..(road.points.len() - 1) {
        let start = road.points[i];
        let end = road.points[i + 1];
        imageproc::drawing::draw_antialiased_line_segment_mut(
            img,
            (
                ((start.tile_x - offset_x) * config.mapping.tile_res + start.x) as i32,
                ((start.tile_y - offset_y) * config.mapping.tile_res + start.y) as i32,
            ),
            (
                ((end.tile_x - offset_x) * config.mapping.tile_res + end.x) as i32,
                ((end.tile_y - offset_y) * config.mapping.tile_res + end.y) as i32,
            ),
            image::Rgb([255, 0, 0]),
            |a, b, f| imageproc::pixelops::interpolate(a, b, f),
        );
    }
}
