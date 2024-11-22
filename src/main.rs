// This is the main source file

use crate::shared::*;
use crate::circles::*;

mod circles;
mod shared;

fn main()
{
    // Preparation
    let args: Vec<String> = std::env::args().collect();
    let mut duration: f32 = 10.0;
    let mut no_stop: bool = false;

    // Checking the flags
    let mut i: usize = 1;
    while i < args.len()
    {
        match args[i].as_str()
        {
            "--help" => { print!("{}", HELP()); std::process::exit(0); }
            "-h" => { print!("{}", HELP()); std::process::exit(0); }

            "--list" => { println!("Avilable drug trip types: \n  1. circles"); std::process::exit(0); } // Placeholder
            "-l" => { println!("Avilable drug trip types: \n  1. circles"); std::process::exit(0); }

            "--no-cancel" => { no_stop = true; }
            "--c" => { no_stop = true; }

            "--duration" => { duration = args[i+1].parse().expect("wrong duration"); i += 1 }
            "-d" => { duration = args[i+1].parse().expect("wrong duration"); i += 1}

            _ => {
                println!("wrong flag: {}", args[i]);
                std::process::exit(1);
            }
        };

        i += 1;
    }

    // Setting things up
    ncurses::initscr();
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    if no_stop { ncurses::raw(); } else {ncurses::cbreak();} // If --no-cancel -c flag was used change input mode to raw
    ncurses::start_color();
    colors_setup(); // From shared.rs

    circles(duration); // From circles.rs

    // Closing the window
    ncurses::endwin();
}

