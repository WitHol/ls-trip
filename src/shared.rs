/*
This file contains different functions, that were needed throught the developement, but are either to general and reusable
to put them in a file created for one specific functionality.
Think of this file as a small library created specifically for this project
*/

use std::f32::consts::PI;

// Custom ncurses colors configuration
pub fn colors_setup()
{
    ncurses::init_color(11, 0x00, 0xbc, 0xd4); // Aqua
    ncurses::init_color(12, 0xff, 0x98, 0x00); // Orange

    ncurses::init_pair(0, ncurses::COLOR_WHITE, ncurses::COLOR_BLACK);
    ncurses::init_pair(1, COLOR_AQUA, ncurses::COLOR_BLACK);
    ncurses::init_pair(2, ncurses::COLOR_GREEN, ncurses::COLOR_BLACK);
    ncurses::init_pair(3, ncurses::COLOR_YELLOW, ncurses::COLOR_BLACK);
    ncurses::init_pair(4, ncurses::COLOR_RED, ncurses::COLOR_BLACK,);
    ncurses::init_pair(5, ncurses::COLOR_MAGENTA, ncurses::COLOR_BLACK);
    ncurses::init_pair(6, COLOR_ORANGE, ncurses::COLOR_BLACK);
    ncurses::init_pair(7, ncurses::COLOR_CYAN, ncurses::COLOR_BLACK);
    ncurses::init_pair(8, ncurses::COLOR_BLUE, ncurses::COLOR_BLACK);
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



// A function for calculation a distance of a point from center
pub fn to_unit(y: i32, x: i32) -> (f32, f32)
{
    return (y as f32/ncurses::LINES() as f32-0.5, x as f32/ncurses::COLS() as f32-0.5);
}


// A function for calculating a difference between two values
pub fn angular_distance(a: &f32, b: &f32) -> f32
{
    let diff = (a - b).abs();
    return match diff > std::f32::consts::PI{
        false => diff,
        true => std::f32::consts::PI*2.0 - diff,
    }
}


// The in which a position is from (0,0), a glorified wrapper around atan2()
pub fn direction(y: &f32, x: &f32) -> f32
{
    let dir = -(-x).atan2(*y);
    return match dir < 0.0 { true => PI*2.0 + dir, false => dir};
}


// A distance of a point from (0,0), a pythagoreas equation
pub fn distance(y: &f32, x: &f32) -> f32
{
    return (y*y + x*x).sqrt();
}
