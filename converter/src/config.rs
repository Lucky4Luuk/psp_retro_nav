pub struct Config {
    pub mapping: ConfigMapping,
    pub style: ConfigStyle,
}

pub struct ConfigMapping {
    pub tile_res: u32,
    pub zoom: u8,
}

pub struct ConfigStyle {
    pub road_color: [u8; 3],
}
