use skia_rs_safe::canvas::RasterCanvas;
use skia_rs_safe::canvas::Surface;
use skia_rs_safe::codec::{ImageEncoder, ImageInfo as CodecImageInfo, PngEncoder};
use skia_rs_safe::core::AlphaType;
use skia_rs_safe::core::Color;
use skia_rs_safe::core::ColorType;
use skia_rs_safe::core::ImageInfo;
use skia_rs_safe::paint::Paint;
use skia_rs_safe::paint::StrokeJoin;
use skia_rs_safe::paint::Style;
use skia_rs_safe::path::PathBuilder;
// use skia_rs_safe::core::{Color, Rect};

use crate::Config;
use crate::mapper::*;

const EARTH_CIRCUMFERENCE_METERS: f64 = 40_075_016_686f64;

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

    // let info = ImageInfo::new(64, 64, ColorType::Rgb888, AlphaType::Unknown).unwrap();

    // temporarily render out a basic test image
    let img_width = (tile_x_max - tile_x_min + 2) * (config.mapping.tile_res as u32);
    let img_height = (tile_y_max - tile_y_min + 2) * (config.mapping.tile_res as u32);
    println!("Rendering image with size ({img_width} x {img_height})");
    // let mut img = image::RgbImage::new(img_width, img_height);
    // let info = ImageInfo::new(
    //     img_width as i32,
    //     img_height as i32,
    //     ColorType::Rgb888,
    //     AlphaType::Unknown,
    // )
    // .unwrap();
    // let surface = Surface::new_raster(&info, None).expect("Failed to create surface!");
    let mut surface = Surface::new_raster_n32_premul(img_width as i32, img_height as i32).unwrap();
    let mut canvas = surface.raster_canvas();

    canvas.clear(Color::BLACK);

    for road in map_tiles.roads {
        draw_road_debug(config, &mut canvas, tile_x_min, tile_y_min, road);
    }

    // surface.save_png("tmp.png").unwrap();
    let pixels = surface.pixels();
    let width = surface.width();
    let height = surface.height();

    let img_info = CodecImageInfo::new(width, height, ColorType::Rgba8888, AlphaType::Premul);
    if let Some(image) =
        skia_rs_safe::codec::Image::from_raster_data(&img_info, pixels, width as usize * 4)
    {
        let encoder = PngEncoder::new();
        match encoder.encode_bytes(&image) {
            Ok(png_data) => {
                if let Err(e) = std::fs::write("tmp.png", &png_data) {
                    eprintln!("Failed to write file: {}", e);
                } else {
                    println!("\nSaved output to: {}", "tmp.png");
                }
            }
            Err(e) => eprintln!("Failed to encode PNG: {}", e),
        }
    }
}

fn draw_road_debug(
    config: &Config,
    canvas: &mut RasterCanvas,
    offset_x: u32,
    offset_y: u32,
    road: Road,
) {
    let mut paint = Paint::new();
    paint.set_color32(Color::from_rgb(
        config.style.road_color[0],
        config.style.road_color[1],
        config.style.road_color[2],
    ));
    paint.set_style(Style::Stroke);
    paint.set_anti_alias(true);
    paint.set_stroke_join(StrokeJoin::Round);
    paint.set_stroke_width(2.0);

    for i in 0..(road.points.len() - 1) {
        let start = road.points[i];
        let end = road.points[i + 1];

        let startx = (start.tile_x - offset_x) * config.mapping.tile_res + start.x;
        let starty = (start.tile_y - offset_y) * config.mapping.tile_res + start.y;

        let endx = (end.tile_x - offset_x) * config.mapping.tile_res + end.x;
        let endy = (end.tile_y - offset_y) * config.mapping.tile_res + end.y;

        let mut path = PathBuilder::new();
        path.move_to(startx as f32, starty as f32);
        path.line_to(endx as f32, endy as f32);

        let lat = (start.lat + end.lat) / 2.0;
        let pixels_per_meter = 1.0 / (meters_per_pixel(config.mapping.zoom, lat) as f32);

        // paint.set_stroke_width(pixels_per_meter * road.width);

        canvas.draw_path(&path.build(), &paint);
    }
}

fn meters_per_pixel(zoom: u8, lat: f64) -> f64 {
    EARTH_CIRCUMFERENCE_METERS * lat.cos() / (2f64.powf(zoom as f64))
}
