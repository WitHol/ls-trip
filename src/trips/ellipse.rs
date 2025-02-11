use rand::Rng;
use crate::shared;

pub fn trip(duration: f32) {
    let ellipse = Ellipse::new();

    // For delta time
    let start= std::time::Instant::now();
    let mut elapsed_time: f32 = 0.0;

    while duration > elapsed_time {
        elapsed_time = std::time::Instant::now().duration_since(start).as_secs_f32();

        ellipse.render(elapsed_time);
    }
}

struct Ellipse {
    speed: f32,
    scale: f32,
}

impl Ellipse {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        return Self {
            speed: rng.gen_range(1.5..3.0),
            scale: rng.gen_range(1.0..3.0),
        }
    }

    // The function, that renders the ellipse
    //
    // -1 < a < 1
    // o = (a - 1/a)/2
    // r = (a + 1/a)/2
    // x^2 + (y-o)^2 = r^2
    // diff = |x^2 + (y-o)^2 - r^2|
    fn render(self: &Self, elapsed_time: f32) {
        let a = (elapsed_time * self.speed) % 2.0 - 1.0;
        let o = (a - 1.0/a)/2.0;
        let r = (a + 1.0/a)/2.0;
    
        for y in 0..ncurses::LINES(){
            for x in 0..ncurses::COLS(){
                let (mut unity, mut unitx) = shared::to_unit(y, x);
                unity *= self.scale;
                unitx *= self.scale;
                
                // Calculating, how far the current pixel is from the ellipse
                let diff = (unitx.powi(2) + (unity-o).powi(2) - r.powi(2)).abs();
    
                // Getting the color pair based on the difference
                let color_pair = match (diff.round() * 2.0) % 9.0 {
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
    
                // Placing the colored space
                ncurses::attr_on(ncurses::COLOR_PAIR(color_pair));
                ncurses::mvaddch(y, x, ' ' as ncurses::ll::chtype);
                ncurses::attroff(ncurses::COLOR_PAIR(color_pair));
            }
        }
        ncurses::refresh();
    }
}
