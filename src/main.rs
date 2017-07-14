extern crate piston_window;
extern crate sdl2_window;
extern crate rand;

use rand::random;

use sdl2_window::Sdl2Window;

use piston_window::{
    MouseCursorEvent,
    CircleArc,
    clear,
    Window
};

use std::ops::{Index,IndexMut};
use std::f64::consts::PI;

type COLOR = [f32;4];
type NUM = f64;

const BALLS : usize = 100;// Should be 800 ()
const MAX_RADIUS : NUM = 45.0;
const DIFFERENCE : NUM = 2.0;
const END_CIRCLE_DRAW : NUM = (PI*2.0)-0.00001;
const BACKGROUND_COLOR : COLOR = [0.14,0.14,0.14,1.0];
const COLOR_ARRAY : [COLOR;5] = [[0.062, 0.356, 0.388,1.0],
                                 [1.0, 0.980, 0.835,1.0],
                                 [1.0, 0.827, 0.305,1.0],
                                 [0.858, 0.619, 0.211,1.0],
                                 [0.741, 0.286, 0.196,1.0]];

fn main(){
    struct Mouse {
        x: NUM,
        y: NUM
    }
    struct Circle {
        x: NUM,
        y: NUM,
        dx: NUM,
        dy: NUM,
        min_radius: NUM,
        shape: CircleArc
    }

    impl Circle {
        pub fn new(x: NUM, y: NUM, dx: NUM, dy: NUM, radius: NUM) -> Self {
            let color : COLOR = COLOR_ARRAY[(random::<f32>() * COLOR_ARRAY.len() as f32).floor() as usize];
            let shape = CircleArc::new(color, radius, 0.0, END_CIRCLE_DRAW);
            Circle { x: x, y: y,
                     dx: dx, dy: dy,
                     min_radius: radius,
                     shape: shape }
        }
    }

    fn update_circle(circle: &mut Circle, width: &NUM, height: &NUM, mouse: &Mouse){

        if circle.x + circle.shape.radius > *width || circle.x - circle.shape.radius < 0.0 {
            circle.dx = -circle.dx;
        }

        if circle.y + circle.shape.radius > *height || circle.y - circle.shape.radius < 0.0 {
            circle.dy = -circle.dy;
        }

        circle.x += circle.dx;
        circle.y += circle.dy;

        // interactivity

        if mouse.x - circle.x < 50.0 &&
            mouse.x - circle.x > -50.0 &&
            mouse.y - circle.y < 50.0 &&
            mouse.y - circle.y > -50.0 {
            if circle.shape.radius < MAX_RADIUS {
                circle.shape.radius += DIFFERENCE;
            }
        } else if circle.shape.radius > circle.min_radius {
            circle.shape.radius -= DIFFERENCE;
        }
    };

    let mut mouse = Mouse { x: 0.0, y: 0.0 };

    let mut window : piston_window::PistonWindow<Sdl2Window>;

    window = piston_window::WindowSettings::new("Dancing Balls",
        [1260,700]).exit_on_esc(true).build().unwrap();

    let width = window.size().width as NUM;
    let height = window.size().height as NUM;

    let mut circle_array : Vec<Circle> = Vec::new();

    fn init(width: &NUM, height: &NUM, circle_array: &mut Vec<Circle>){
        *circle_array = Vec::new();

        let (mut radius, mut x, mut y, mut dx, mut dy) : (NUM,NUM,NUM,NUM,NUM);

        for i in 0..BALLS {
            radius = random::<NUM>() * 5.0 + 1.0;
            x = random::<NUM>() * (width - radius * 3.0) + radius;
            y = random::<NUM>() * (height - radius * 3.0) + radius;
            dx = random::<NUM>() - 0.5;
            dy = random::<NUM>() - 0.5;

            circle_array.push(Circle::new(x, y, dx, dy, radius));
        }
    }

    init(&width, &height, &mut circle_array);
    while let Some(e) = window.next() {
        e.mouse_cursor(|x,y| {
            mouse.x = x;
            mouse.y = y;
        });
        window.draw_2d(&e, |c, g|{
            clear(BACKGROUND_COLOR, g);
            for i in 0..circle_array.len() {
                update_circle(circle_array.index_mut(i), &width, &height, &mouse);
                // draw circle
                circle_array.index(i).shape.draw_tri([circle_array.index(i).x,circle_array.index(i).y,1.0,1.0], &c.draw_state, c.transform, g);
            }
        });
    }
}
