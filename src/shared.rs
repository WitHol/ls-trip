// Custom colors configuration
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


pub fn angular_distance(a: &f32, b: &f32) -> f32
{
    let diff = (a - b).abs();
    return match diff > std::f32::consts::PI{
        false => diff,
        true => std::f32::consts::PI*2.0 - diff,
    }
}


// The only purpose of this function is to hold the help message
pub fn HELP() -> String
{
String::from("
Simulate a drug trip whenever you mistype ls for lsd
Usage: lsd/ls-trip <flag> [...]

Avilable flags:
    -h, --help              print this menu
    -l, --list              list drug trip types
    -t, --type              drug trip type name
    -T, --type-number       drug trip type number
    -d, --duration          duration of the drug trip in seconds
    -c, --no-cancel         disable the user's ability to stop the program with ctrl-c
")
}