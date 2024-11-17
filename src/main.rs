use ncurses::init_color;
use shared::colors_setup;
use std::collections::{hash_map, HashMap};

mod shared;
mod circles;

fn main()
{
    // Detecting if the program was ran with the --help flag
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from("--help"))
    {
        println!("This is a joke shell utility, that simulates a drug trip whenever you mistype 'ls' for 'lsd'");
        return;
    }

    // Setting things up
    ncurses::initscr();
    ncurses::noecho();
    ncurses::cbreak();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::start_color();

    colors_setup();

    circles::circles(1000000.0);

    // Closing the window
    ncurses::endwin();
}

