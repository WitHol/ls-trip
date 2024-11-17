use ncurses::chtype;
use rand::Rng;

use crate::shared::*;

extern crate rand;

pub fn circles(mut duration: f32)
{
    let mut circles: Vec<Circle> = vec![];
    let amount: i8 = rand::thread_rng().gen_range(8..13);
    
    for _ in (0..amount)
    {
        circles.push(Circle::new());
    }
    
    // Preparing a variable for calculating delta time
    let mut previous= std::time::Instant::now();
   
    while duration > 0.0
    {
        // Calculating the delta time
        let now = std::time::Instant::now();
        let delta_time: f32 = now.duration_since(previous).as_secs_f32();
        previous = now;
        duration -= delta_time;

        tick_circles(&mut circles, &delta_time);
        render_circles(&circles);
    }
}

fn render_circles(circles: &Vec<Circle>)
{

    for y in 0..ncurses::LINES() as i16
    {
        for x in 0..ncurses::COLS() as i16
        {
            ncurses::mv(y as i32, x as i32);
            let (unity,unitx) = unit_pos(y, x);
            let mut level: f32 = 0.0;

            for circle in circles.iter()
            {
                level += 1.0/circle.distance(unity, unitx);
            }

            let pair = match ((level*0.6).round() % 9.0)
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

            ncurses::attron(ncurses::COLOR_PAIR(pair));
            ncurses::addch('#' as chtype); // #
            ncurses::attroff(ncurses::COLOR_PAIR(pair));
        }
    }

    ncurses::refresh();
}

fn tick_circles(circles: &mut Vec<Circle>, delta_time: &f32)
{
    for circle in circles.iter_mut()
    {
        circle.tick(delta_time);
    }
}

fn unit_pos(y: i16, x: i16) -> (f32, f32)
{
    return (y as f32/ncurses::LINES() as f32-0.5, x as f32/ncurses::COLS() as f32-0.5);
}

impl Circle
{
    fn new() -> Circle
    {
        let mut rng = rand::thread_rng();

        let mut circle: Circle = Circle{
            angle: rng.gen_range(0.0..2.0*std::f32::consts::PI),
            offset: rng.gen_range(0.0..0.2),
            delta_angle: 0.0,
            delta_offset: 0.0,
            x: 0.0,
            y: 0.0,
        };

        circle.calculate_pos();

        return circle;
    }

    fn calculate_pos(self: &mut Circle)
    {
        self.y = self.offset * self.angle.cos();
        self.x = self.offset * self.angle.sin();
    }

    fn tick(self: &mut Circle, delta_time: &f32)
    {
        if rand::thread_rng().gen_range(0.0..1.0) < 60.0*delta_time
        {
            self.delta_angle += rand::thread_rng().gen_range(-0.2..0.2);
            if self.delta_angle.abs() > std::f32::consts::PI * 1.0 { self.delta_angle *= 0.9; }

            self.delta_offset += rand::thread_rng().gen_range(-0.2..0.2);
            if self.offset.abs() > 1.0 { self.delta_offset -= 0.1 }
            if self.offset.abs() < 0.1 { self.delta_offset += 0.1 }
            self.delta_offset *= 0.9;
        }

        self.angle += self.delta_angle * delta_time;
        self.offset += self.delta_offset * delta_time;

        self.calculate_pos();
    }

    fn clone(self: &Circle) -> Circle
    {
        let mut circle: Circle = Circle{
            angle: self.angle,
            offset: self.offset,
            delta_angle: self.delta_angle,
            delta_offset: self.delta_offset,
            x: self.x,
            y: self.y,
        };

        return circle;
    }

    fn distance(self: &Circle, y:f32, x:f32) -> f32
    {
        return ((self.y-y).powi(2) + (self.x-x).powi(2)).sqrt()
    }
}

struct Circle
{
    angle: f32,
    offset: f32,
    delta_angle: f32,
    delta_offset: f32,
    x: f32,
    y: f32,
}