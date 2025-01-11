// This file contains functions used to create the "circles" drug trip type
// The only thing, that should be used outside of this file is the main() function,
// everyting else is only used here

use rand::Rng;
use crate::shared;
use std::f32::consts::PI;

// The main function
pub fn main(duration: f32)
{
    let mut center = Center::new();

    let start = std::time::Instant::now();
    let mut elapsed_time: f32 = 0.0;
   
    while duration > elapsed_time
    {
        elapsed_time = std::time::Instant::now().duration_since(start).as_secs_f32();

        center.tick(elapsed_time);
        center.render();
    }
}

// The center struct with implementations of the functions
// Everthing is measured using unit terminal dimentions, (-1, -1) is in the top-left corner, (1,1) is bottom right
// This is a separate struct with its own functions purely for organization purposes 
impl Center
{
    // A function for creating a center (only used once)
    fn new() -> Center
    {
        let mut rng = rand::thread_rng();

        let mut cuts = Vec::<f32>::new();
        let cut_count = rng.gen_range(4..8);
        for i in 0..cut_count
        {
            cuts.push(i as f32 * PI*2.0 / cut_count as f32);
        }

        Center{
            cuts: cuts,
            cut: rng.gen_range(0.3..0.7),
            compactness: rng.gen_range(12.0..18.0),
            
            rotation: 0.0,
            shift: 0.0,
            
            delta_rotation: rng.gen_range(0.0..0.1),
            delta_shift: rng.gen_range(0.5..0.7),
        }
    }

    // A function for rendering thes
    fn render(self: &Center)
    {
        for y in 0..ncurses::LINES()
        {
            for x in 0..ncurses::COLS()
            {
                ncurses::mv(y,x);
                let (unity, unitx) = shared::to_unit(y,x);

                let mut level: f32 = shared::distance(&unity, &unitx);
                level += self.shift;
                
                for cut in self.cuts.iter()
                {
                    let dir = shared::direction(&unity, &unitx);
                    let mut dist = shared::angular_distance(cut, &dir);
                    dist = (0.8 / (dist + 0.4) - 0.4) * self.cut;
                    if dist > 0.0 { level += dist; }
                }

                let pair = match (level * self.compactness).round() % 9.0
                {
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
                ncurses::addch('#' as ncurses::ll::chtype);
                ncurses::attroff(ncurses::COLOR_PAIR(pair));
            }
        }

        
        ncurses::refresh();
    }

    // A functions for updating the parameters of a center
    fn tick(self: &mut Center, elapsed_time: f32)
    {
        self.rotation = self.delta_rotation * elapsed_time;
        self.shift = self.delta_shift * elapsed_time;
    }


}
struct Center
{
    cuts: Vec<f32>, 
    cut: f32,
    compactness: f32,

    delta_rotation: f32,
    delta_shift: f32,

    rotation: f32,
    shift: f32,
}
