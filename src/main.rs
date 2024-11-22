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
    // --help -h
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h"))
    {
        print!("{:}", HELP());
        return;
    }
    // --list -l
    if args.contains(&String::from("--list")) || args.contains(&String::from("-l"))
    {
        // Placeholder, this will be completly redone
        println!("Avilable drug trip types: ");
        println!("  1. circles");

        return;
    }
    // --no-cancel -c
    if args.contains(&String::from("--no-cancle")) || args.contains(&String::from("-c"))
    {
        no_stop = true;
    }
    // --duration -d
    if args.contains(&String::from("--duration"))
    {
        duration = args[args.iter().position(|a| a == "--duration").unwrap() + 1]
            .trim().parse().expect("Wrong duration!");
    }
    else if args.contains(&String::from("-d"))
    {
        duration = args[args.iter().position(|a| a == "-d").unwrap() + 1]
            .trim().parse().expect("Wrong duration!");
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

