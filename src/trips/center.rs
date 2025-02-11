use rand::Rng;
use crate::shared;
use std::f32::consts::PI;

pub fn trip(duration: f32) {
    let center = Center::new();

    // For delta time
    let start = std::time::Instant::now();
    let mut elapsed_time: f32 = 0.0;
   
    while duration > elapsed_time {
        elapsed_time = std::time::Instant::now().duration_since(start).as_secs_f32();

        center.render(&elapsed_time);
    }
}

struct Center {
    cuts: Vec<f32>, 
    compactness: f32,
    cut: f32,
    shift: f32,
}

impl Center {
    fn new() -> Center {
        let mut rng = rand::thread_rng();

        // Calculating the positions of the cuts
        let cut_count = rng.gen_range(4..8);
        let mut cuts = Vec::<f32>::new();
        for i in 0..cut_count {
            cuts.push(i as f32 * PI*2.0 / cut_count as f32);
        }

        Center{
            cuts,
            compactness: rng.gen_range(12.0..18.0),
            cut: rng.gen_range(0.7..1.3),
            
            shift: rng.gen_range(0.6..0.9),
        }
    }

    // A function for rendering the pattern for a given time
    fn render(self: &Center, elapsed_time: &f32) {
        let cut = (elapsed_time * 3.0).sin()/2.0 + 0.7 * self.cut;

        for y in 0..ncurses::LINES() {
            for x in 0..ncurses::COLS() {
                let (unity, unitx) = shared::to_unit(y,x);

                // Calculating the level through several steps
                let mut level = shared::distance(&unity, &unitx);
                level += self.shift * elapsed_time;
                for cut_ in self.cuts.iter() {
                    let dir = shared::direction(&unity, &unitx);
                    let dist = shared::angular_distance(cut_, &dir);
                    level += cut / (dist + 0.5) - cut * 0.5;
                }

                // Getting the color based on the level
                let pair = match (level * self.compactness).round() % 9.0 {
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

                // Displaying the color
                ncurses::attr_on(ncurses::COLOR_PAIR(pair));
                ncurses::mvaddch(y, x, ' ' as ncurses::ll::chtype);
                ncurses::attroff(ncurses::COLOR_PAIR(pair));
            }
        }
        ncurses::refresh();
    }
}
