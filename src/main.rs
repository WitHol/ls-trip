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
            "--help" | "-h" => { 
                print!("{}", HELP()); std::process::exit(0); 
            }

            "--list" | "-l" => {  // Placeholder
                println!("Avilable drug trip types: \n  1. circles"); 
                std::process::exit(0); 
            }

            "--no-cancel" | "-c" => { 
                no_stop = true; 
            }

            "--duration" | "-d" => {
                if i == args.len()-1 
                { 
                    println!("no duration provided");
                    std::process::exit(1);
                }
                duration = match args[i+1].parse()
                { 
                    Ok(T) => {T} 
                    Err(_) => {
                        println!("wrong duration: {}", args[i+1]);
                        std::process::exit(1);
                    }
                };
                i += 1;
            }

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

