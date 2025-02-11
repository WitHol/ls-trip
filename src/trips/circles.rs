// This file contains functions used to create the "circles" drug trip type
// The only thing, that should be used outside of this file is the main() function,
// everyting else is only used here

use rand::Rng;
use crate::shared;

// The main function
pub fn trip(duration: f32) {
    let circles_count: i8 = rand::thread_rng().gen_range(8..13);
    let mut circles: Vec<Circle> = vec![];
    
    for _ in 0..circles_count {
        circles.push(Circle::new());
    }
    
    let start= std::time::Instant::now();
    let mut elapsed_time: f32 = 0.0;

    while duration > elapsed_time {
        elapsed_time = std::time::Instant::now().duration_since(start).as_secs_f32();

        tick_circles(&mut circles, &elapsed_time);
        render_circles(&circles);
    }
}


// The circle struct with implementations of the functions
// Everthing is measured using unit terminal dimentions, (-1, -1) is in the top-left corner, (1,1) is bottom right
struct Circle {
    angle: f32,
    offset: f32,
    delta_angle: f32,
    x: f32,
    y: f32,

    random_factor: f32,
}

impl Circle {
    // A function for creating a new circle with randomized parameters
    fn new() -> Circle {
        let mut rng = rand::thread_rng();

        let mut circle: Circle = Circle{
            delta_angle: rng.gen_range(-2.0..2.0),
            random_factor: rng.gen_range(1.0..1000.0),
            angle: 0.0,
            offset: 0.0,
            x: 0.0,
            y: 0.0,
        };

        circle.calculate_pos();

        return circle;
    }
    
    // A function for updating the position of a circle, it does not output anything, it only changes struct values
    fn tick(self: &mut Circle, elapsed_time: &f32) {
        self.angle = elapsed_time * self.delta_angle;
        self.offset = (elapsed_time + self.random_factor).sin() * 0.6;
        
        self.calculate_pos();
    }
    
    // A function for calculating the distance from the center of a circle to a certain point
    fn distance(self: &Circle, y:f32, x:f32) -> f32 {
        return ((self.y-y).powi(2) + (self.x-x).powi(2)).sqrt()
    }

    // A function for calculating the position, does not output anything and is only used by other functions
    fn calculate_pos(self: &mut Circle) {
        self.y = self.offset * self.angle.cos();
        self.x = self.offset * self.angle.sin();
    }
}


// The funtion used for rendering the circles to the screen
fn render_circles(circles: &Vec<Circle>) {
    for y in 0..ncurses::LINES() {
        for x in 0..ncurses::COLS() {
            ncurses::mv(y, x);
            let (unity,unitx) = shared::to_unit(y, x);
            let mut level: f32 = 0.0;

            for circle in circles.iter() {
                level += 1.0/circle.distance(unity, unitx);
            }

            let pair = match (level*0.6).round() % 9.0 {
                0.0 => shared::PAIR_WHITE,
                1.0 => shared::PAIR_AQUA,
                2.0 => shared::PAIR_GREEN,
                3.0 => shared::PAIR_YELLOW,
                4.0 => shared::PAIR_RED,
                5.0 => shared::PAIR_MAGENTA,
                6.0 => shared::PAIR_ORANGE,
                7.0 => shared::PAIR_CYAN,
                8.0 => shared::PAIR_BLUE,
                _ => shared::PAIR_WHITE,
            };

            ncurses::attron(ncurses::COLOR_PAIR(pair));
            ncurses::addch(' ' as ncurses::ll::chtype);
            ncurses::attroff(ncurses::COLOR_PAIR(pair));
        }
    }

    ncurses::refresh();
}

// A function, that calls tick() for all the circles
fn tick_circles(circles: &mut Vec<Circle>, elapsed_time: &f32) {
    for circle in circles.iter_mut() {
        circle.tick(elapsed_time);
    }
}
