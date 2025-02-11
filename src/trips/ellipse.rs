use rand::Rng;
use crate::shared;

pub fn trip(duration: f32) {
    let ellipse = Ellipse::new();

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
    compactness: f32,
}

impl Ellipse {
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
                let x_ = self.scale * (x as f32 / ncurses::COLS() as f32 - 0.5);
                let y_ = self.scale * (y as f32 / ncurses::LINES() as f32 - 0.5);
                
                let diff = (x_.powi(2) + (y_-o).powi(2) - r.powi(2)).abs();
    
                let pair = match (diff.round() * self.compactness) % 9.0 {
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
    
                ncurses::attr_on(ncurses::COLOR_PAIR(pair));
                ncurses::mvaddch(y, x, ' ' as ncurses::ll::chtype);
                ncurses::attroff(ncurses::COLOR_PAIR(pair));
                
            }
        }
    
        ncurses::refresh();
    }

    fn new() -> Self {
        let mut rng = rand::thread_rng();
        return Self {
            speed: rng.gen_range(1.5..3.0),
            compactness: rng.gen_range(1.5..2.5),
            scale: rng.gen_range(1.5..3.0),
        }
    }
}
