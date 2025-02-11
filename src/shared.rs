/*
This file contains different functions, that were needed throught the developement, 
but are either to general and reusable, to put them in a single file,
or I didn't want to clutter some other source file
*/

use std::f32::consts::PI;

// Custom ncurses colors configuration
pub fn colors_setup() {
    ncurses::init_color(11, 0x00, 0xbc, 0xd4); // Aqua
    ncurses::init_color(12, 0xff, 0x98, 0x00); // Orange

    ncurses::init_pair(0, ncurses::COLOR_BLACK, ncurses::COLOR_WHITE);
    ncurses::init_pair(1, ncurses::COLOR_BLACK, COLOR_AQUA);
    ncurses::init_pair(2, ncurses::COLOR_BLACK, ncurses::COLOR_GREEN);
    ncurses::init_pair(3, ncurses::COLOR_BLACK, ncurses::COLOR_YELLOW);
    ncurses::init_pair(4, ncurses::COLOR_BLACK, ncurses::COLOR_RED);
    ncurses::init_pair(5, ncurses::COLOR_BLACK, ncurses::COLOR_MAGENTA);
    ncurses::init_pair(6, ncurses::COLOR_BLACK, COLOR_ORANGE);
    ncurses::init_pair(7, ncurses::COLOR_BLACK, ncurses::COLOR_CYAN);
    ncurses::init_pair(8, ncurses::COLOR_BLACK, ncurses::COLOR_BLUE);
}

pub const COLOR_AQUA: i16 = 11;
pub const COLOR_ORANGE: i16 = 12;

pub const PAIR_WHITE: i16 = 0;
pub const PAIR_AQUA: i16 = 1;
pub const PAIR_GREEN: i16 = 2;
pub const PAIR_YELLOW: i16 = 3;
pub const PAIR_RED: i16 = 4;
pub const PAIR_MAGENTA: i16 = 5;
pub const PAIR_ORANGE: i16 = 6;
pub const PAIR_CYAN: i16 = 7;
pub const PAIR_BLUE: i16 = 8;

pub const COLOR_PAIRS: [i8; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

// A function for calculating unit position from a terminal one
pub fn to_unit(y: i32, x: i32) -> (f32, f32) {
    return (y as f32/ncurses::LINES() as f32-0.5, x as f32/ncurses::COLS() as f32-0.5);
}

// A function for calculating a difference between two angles
pub fn angular_distance(a: &f32, b: &f32) -> f32 {
    let diff = (a - b).abs();
    return match diff > PI{
        false => diff,
        true => PI*2.0 - diff,
    }
}

// The in which a position is from (0,0), a glorified wrapper around atan2()
pub fn direction(y: &f32, x: &f32) -> f32 {
    let dir = -(-x).atan2(*y);
    return match dir < 0.0 { true => std::f32::consts::PI*2.0 + dir, false => dir};
}

// A distance of a point from (0,0) with pythagoras' theorem
pub fn distance(y: &f32, x: &f32) -> f32 {
    return (y*y + x*x).sqrt();
}

// This is a string, that contains the help message
pub const HELP: &str = 
"Simulate a drug trip whenever you mistype ls for lsd
Usage: lsd/ls-trip [flag] [...]

Avilable flags:
    -h, --help              print this message
    -l, --list              list drug trip types
    -t, --type              set the drug trip type by name
    -T, --type-number       set the drug trip type by number
    -d, --duration          set the duration of the drug trip (in seconds)
    -c, --no-cancel         disable the user's ability to stop the program with ctrl-c";

// A custom "hash map", that assigns trip numbers to names
pub const DRUG_TRIPS: [(i16, &str); 3]  = [
    (1, "circles"),
    (2, "center"),
    (3, "ellipse"),
];
