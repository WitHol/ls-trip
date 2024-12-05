use rand::Rng;
use crate::shared::*;
use std::f32::consts::PI;

extern crate rand;

pub fn center(mut duration: f32)
{
    let mut center = Center::new();

    // Preparing a variable for calculating delta time
    let mut previous= std::time::Instant::now();
   
    while duration > 0.0
    {
        let now = std::time::Instant::now();
        let delta_time: f32 = now.duration_since(previous).as_secs_f32();
        previous = now;
        duration -= delta_time;

        center.tick(delta_time);
        center.render();
    }
}

impl Center
{
    fn new() -> Center
    {
        let mut rng = rand::thread_rng();

        let mut cuts = Vec::<f32>::new();
        let cut_count = rng.gen_range(5..8);
        for i in 0..cut_count
        {
            cuts.push(i as f32 * PI*2.0 / cut_count as f32);
        }

        Center{
            cuts: cuts,
            
            rotation: 0.0,
            cut: rng.gen_range(0.3..0.7),
            shift: 0.0,
            
            compactness: rng.gen_range(12.0..18.0),
            delta_rotation: rng.gen_range(0.0..0.1),
            delta_shift: rng.gen_range(0.2..0.3),
        }
    }

    fn render(self: &Center)
    {
        for y in 0..ncurses::LINES()
        {
            for x in 0..ncurses::COLS()
            {
                ncurses::mv(y,x);
                let (unity, unitx) = to_unit(y,x);

                let mut level: f32 = self.distance(&unity, &unitx);
                level += self.shift;
                
                for cut in self.cuts.iter()
                {
                    let dir = self.direction(&unity, &unitx);
                    let mut dist = angular_distance(cut, &dir);
                    dist = (0.4 - dist) * self.cut;
                    if dist > 0.0 { level += dist; }
                }

                let pair = match (level * self.compactness).round() % 9.0
                {
                    0.0 => PAIR_WHITE,
                    1.0 => PAIR_AQUA,
                    2.0 => PAIR_GREEN,
                    3.0 => PAIR_YELLOW,
                    4.0 => PAIR_RED,
                    5.0 => PAIR_MAGENTA,
                    6.0 => PAIR_ORANGE,
                    7.0 => PAIR_CYAN,
                    8.0 => PAIR_BLUE,
                    _ => PAIR_WHITE,
                };

                ncurses::attr_on(ncurses::COLOR_PAIR(pair));
                ncurses::addch('#' as ncurses::ll::chtype);
                ncurses::attroff(ncurses::COLOR_PAIR(pair));
            }
        }

        
        ncurses::refresh();
    }

    fn direction(self: &Center, y: &f32, x: &f32) -> f32
    {
        let dir = -(-x).atan2(*y);
        return match dir < 0.0 { true => PI*2.0 + dir, false => dir};
    }

    fn tick(self: &mut Center, delta_time: f32)
    {
        self.rotation += self.delta_rotation * delta_time;
        self.shift += self.delta_shift * delta_time;
    }

    fn distance(self: &Center, y: &f32, x: &f32) -> f32
    {
        return (y*y + x*x).sqrt();
    }
}

struct Center
{
    cuts: Vec<f32>, 

    shift: f32,
    rotation: f32,
    cut: f32,
    compactness: f32,
    delta_shift: f32,
    delta_rotation: f32,
}