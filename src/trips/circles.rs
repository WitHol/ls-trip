use rand::Rng;
use crate::shared;

// The main function
pub fn trip(duration: f32) {
    // Creating the circles
    let circles_count: i8 = rand::thread_rng().gen_range(8..13);
    let mut circles: Vec<Circle> = vec![];
    for _ in 0..circles_count {
        circles.push(Circle::new());
    }
    
    // For delta time
    let start= std::time::Instant::now();
    let mut elapsed_time: f32 = 0.0;

    while duration > elapsed_time {
        elapsed_time = std::time::Instant::now().duration_since(start).as_secs_f32();

        for circle in circles.iter_mut() {
            circle.tick(&elapsed_time);
        }

        render_circles(&circles);
    }
}

fn render_circles(circles: &Vec<Circle>) {
    for y in 0..ncurses::LINES() {
        for x in 0..ncurses::COLS() {
            ncurses::mv(y, x);
            let (unity,unitx) = shared::to_unit(y, x);

            // Calculating the level of the current digit
            // As the level gets higher the digit goes through different colors in a loop
            let mut level: f32 = 0.0;
            for circle in circles.iter() {
                level += 1.0 / circle.distance_from(unity, unitx);
            }

            // Calculating the color based on that level
            let color_pair = match (level*0.6).round() % 9.0 {
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

            // Coloring the current digit
            ncurses::attron(ncurses::COLOR_PAIR(color_pair));
            ncurses::addch(' ' as ncurses::ll::chtype);
            ncurses::attroff(ncurses::COLOR_PAIR(color_pair));
        }
    }

    ncurses::refresh();
}

// x and y are using unit coords
struct Circle {
    velocity: f32,
    randomness_factor: f32,
    x: f32,
    y: f32,
}

impl Circle {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        return Self {
            velocity: rng.gen_range(-2.0..2.0),
            randomness_factor: rng.gen_range(1.0..1000.0),
            x: 0.0,
            y: 0.0,
        };
    }
    
    // A function, that calculates the position of a circle at a given time
    fn tick(self: &mut Self, elapsed_time: &f32) {
        let angle = elapsed_time * self.velocity;
        let offset = (elapsed_time + self.randomness_factor).sin() * 0.6;
        
        self.y = offset * angle.cos();
        self.x = offset * angle.sin();
    }

    fn distance_from(self: &Self, y:f32, x:f32) -> f32 {
        return ((self.y-y).powi(2) + (self.x-x).powi(2)).sqrt()
    }
}
