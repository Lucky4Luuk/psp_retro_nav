#![no_std]
#![no_main]
#![allow(static_mut_refs)]

#[macro_use]
extern crate alloc;

use alloc::string::ToString;

use crate::io::start_reading_tile;

pub mod gps;
pub mod io;

psp::module!("nav_soft", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    psp::dprintln!("hi!");

    // gps::init_gps();

    // let path = "ms0:/PSP/GAME/\0";
    // for entry in io::read_dir(path) {
    //     psp::dprintln!("Entry: {entry:?}");
    // }

    start_reading_tile("ms0:/PSP/GAME/psp_retro_nav/test.qoi\0");

    psp::dprintln!("done!");
}
