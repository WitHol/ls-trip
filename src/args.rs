/*
This file containst a pure, public main funtion, that parses the arguments and adjusts given
variables, as well as a few other private functions to prevent code duplication
*/

use crate::shared::{self, DRUG_TRIPS};

macro_rules! exec_if_provided {
    ( $fun:expr, $val:expr, $args:expr, $i:expr, $mess:expr) => {
        $fun($val, $args.get($i+1).expect($mess), &mut $i)
    }
}

macro_rules! exec_if_provided_and_last {
    ($fun:expr, $val:expr, $args:expr, $i:expr, $j:expr, $mess:expr) => {
        if $j == $args[$i].len()-1 {
            exec_if_provided!($fun, $val, $args, $i, $mess);
        } else {
            panic!("flags that requires a value have to be at the end of the combination");
        }
    };
}

// The main function
pub fn parse(args: &Vec<String>, duration: &mut f32, no_cancel: &mut bool, trip_type: &mut i16){
    let mut i: usize = 1;
    while i < args.len() {
        if &args[i][0..2] == "--" { // Long flags starting with --
            match args[i].as_str() {
                "--help" => arg_help(),
                "--list" => arg_list(),
                "--no-cancel" => arg_no_cancel(no_cancel),
                "--duration" => exec_if_provided!(arg_duration, duration, args, i, "duration not provided"),
                "--type" => exec_if_provided!(arg_type, trip_type, args, i, "trip type not provided"),
                "--type-number" => exec_if_provided!(arg_type_number, trip_type, args, i, "trip type number not provided"),
                other_flag => panic!("wrong flag: {}", other_flag)
            };
        }
        else if &args[i][0..1] == "-" { // Short flags starting with -
            for j in 1..args[i].len() {
                let char = &args[i][j..j+1];
                match char {
                    "h" => arg_help(),
                    "l" => arg_list(),
                    "c" => arg_no_cancel(no_cancel),
                    "d" => exec_if_provided_and_last!(arg_duration, duration, args, i, j, "duration not provided"),
                    "t" => exec_if_provided_and_last!(arg_type, trip_type, args, i, j, "trip type not provided"),
                    "T" => exec_if_provided_and_last!(arg_type_number, trip_type, args, i, j, "trip type number not provided"),
                    other_flag => panic!("wrong flag: {}", other_flag)
                }
            }
        }
        else { // Arguments, that aren't flags
            panic!("wrong argument: {}", args[i]);
        }

        i += 1;
    }
}


// The following functions directly correspond to the flags
fn arg_help() {
    println!("{}", shared::HELP);
    std::process::exit(0);
}

fn arg_list() {
    println!("Avilable drug trip types:");
    for (number, name) in shared::DRUG_TRIPS.iter() {
        println!("{number} - {name}");
    }

    std::process::exit(0);
}

fn arg_no_cancel(no_cancel: &mut bool) {
    *no_cancel = false;
}

fn arg_type(trip_type: &mut i16, next_arg: &str, i: &mut usize) {
    for (number, name) in DRUG_TRIPS.iter(){
        if *name == next_arg {
            *trip_type = *number;
            *i += 1;
            return;
        }
    }
    
    panic!("no such drug trip");
}

fn arg_type_number(trip_type: &mut i16, next_arg: &str, i: &mut usize) {
    *trip_type = next_arg.parse::<i16>()
        .expect("not a number");

    *i += 1;
}

fn arg_duration(duration: &mut f32, next_arg: &str, i: &mut usize) {
    *duration = next_arg.parse()
        .expect("wrong duration");

    *i += 1;
}
