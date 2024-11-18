use std::io::Read;

use crate::shared::*;
use crate::circles::*;

mod circles;
mod shared;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let mut duration: f32 = 5.0;
    let mut no_stop: bool = false;

    // Checking the flags
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h"))
    {
        match std::fs::File::open("/home/withol/projects/ls-trip/src/assets/help.txt"){
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Error reading the help file");
                println!("{}", contents);
            }

            Err(err) => eprintln!("Error opening the help file: {}", err)
        }
        return;
    }

    if args.contains(&String::from("--list")) || args.contains(&String::from("-l"))
    {
        // Placeholder, this will be completly redone
        println!("Avilable drug trip types: ");
        println!("  1. circles");

        return;
    }

    if args.contains(&String::from("--no-cancle")) || args.contains(&String::from("-c"))
    {
        no_stop = true;
    }

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
    if no_stop { ncurses::raw(); } else {ncurses::cbreak();}
    ncurses::start_color();
    colors_setup(); // From shared.rs

    circles(duration); // From circles.rs

    // Closing the window
    ncurses::endwin();
}

