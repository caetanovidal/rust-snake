extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

use rand::Rng;
use rand::thread_rng;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}


struct Game{
    gl: GlGraphics,
    snake: Snake,
    score: u32,
    food: Food,
    just_eaten: bool
}

impl Game {
    fn render(&mut self, arg: &RenderArgs){
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl|{
            graphics::clear(BLACK, gl);
        });

        self.food.render(&mut self.gl, arg, 20);
        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self) -> bool{

        self.just_eaten = self.food.update(&self.snake);

        if self.snake.update(self.just_eaten){
            return true;
        };

        if self.just_eaten{
            self.score += 1;

            let mut rng = thread_rng();
            loop {
                let x = rng.gen_range(0, 20);
                let y = rng.gen_range(0, 20);

                if !self.snake.is_collide(x, y){
                    self.food = Food { x: x, y: y };
                    break;
                }
            }
            self.just_eaten = false
        }


        return false;
    }

    fn pressed(&mut self, btn:&Button){
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            _ => last_direction
        };
    }
}

struct Snake{
    body: LinkedList<(i32, i32)>,
    dir: Direction
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x,y)|{
                graphics::rectangle::square(
                    (x*20) as f64, 
                    (y*20) as f64, 
                    20_f64)

            })
            .collect();

        


        gl.draw(args.viewport(), |c, gl|{
            let transform = c.transform;
            squares.into_iter().for_each(|square|{
                graphics::rectangle(BLUE, square, transform, gl);
            })
        });
    }

    fn update(&mut self, just_eaten: bool) -> bool{
        let mut new_head: (i32, i32) = (*self.body.front().unwrap()).clone();

        match self.dir {
            Direction::Right => new_head.0 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        if !just_eaten{
            self.body.pop_back().unwrap();
        }


        if self.is_collide(new_head.0, new_head.1){
            return true;
        }

        if self.wall_collide(new_head.0, new_head.1){
            return true;
        }

        self.body.push_front(new_head);
        
        
        return false;
    }

    fn is_collide(&self, x: i32, y: i32) -> bool{
        self.body.iter().any(|p| p.0 == x && p.1 == y)
    }

    fn wall_collide(&self, x: i32, y: i32) -> bool{
        x < 0 || x > 39 || y < 0 || y > 29
    }


}

pub struct Food {
    x: i32,
    y: i32
}

impl Food{
    fn update(&mut self, s: &Snake) -> bool{
        let front = s.body.front().unwrap();

        if front.0 == self.x && front.1 == self.y {
            true
        }
        else{
            false
        }
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, width: i32){
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl|{
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("snake", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let list: LinkedList<(i32, i32)> = LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter());
    let food = Food { x: 2, y: 2 };

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake { body: list, dir: Direction::Down },
        score: 0,
        food: food,
        just_eaten: false
    };

    let mut events = Events::new(EventSettings::new()).ups(20);
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_) = e.update_args() {
            if game.update(){
                break;
            };
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }

    }

    println!("Score: {}", game.score);
}
