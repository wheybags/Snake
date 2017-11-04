extern crate termion;
extern crate rand;

use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout};
use std::time::Duration;
use std::thread;
use rand::distributions::{IndependentSample, Range};

#[derive(Clone, PartialEq, Eq)]
enum Tile
{
    Wall,
    Floor,
    Food,
    Snake,
    SnakeHead
}

impl Default for Tile
{
    fn default() -> Tile { Tile::Food }
}

#[derive(Clone, PartialEq, Eq)]
struct Point
{
    x: i32,
    y: i32,
}

#[derive(Clone, PartialEq, Eq)]
enum Dir
{
    Up,
    Right,
    Down,
    Left
}

struct Snake
{
    segments: Vec<Point>,
    dir: Dir
}

struct World
{
    snake: Snake,
    screen: Vec<Tile>,
    width: usize,
    height: usize,
    score: i32
}

fn init_world(width: usize, height: usize, init_size: usize) -> World
{
    let mut world = World { snake: Snake { segments: Vec::new(), dir: Dir::Right },
                            screen: vec![Tile::Floor; width * height],
                            width,
                            height,
                            score: 0 };
    
    {
        let screen = &mut world.screen;

        for y in 0..height
        {
            for x in 0..width
            {
                if x == 0 || x == width-1 || y == 0 || y == height-1
                {
                    screen[x + y*width] = Tile::Wall;
                }
            }
        }
    }
    
    {
        let snake = &mut world.snake;
        
        snake.dir = Dir::Right;

        for x in (width/2 - init_size/2)..(width/2 - init_size/2 + init_size)
        {
            snake.segments.push(Point { x: x as i32, y: (height / 2) as i32 });
        }

    }
    
    place_food(&mut world);
    
    return world;
}

fn place_food(world: &mut World)
{
    let mut try_point: Point;
    
    let mut rng = rand::thread_rng();
    
    loop
    {
        try_point = Point { x: Range::new(1, world.width  as i32 - 1).ind_sample(&mut rng),
                           y: Range::new(1, world.height as i32 - 1).ind_sample(&mut rng) };

        if world.screen[(try_point.x + try_point.y * world.width as i32) as usize] == Tile::Floor && !world.snake.segments.contains(&try_point)
        {
            break;
        }
    }

    world.screen[(try_point.x + try_point.y * world.width as i32) as usize] = Tile::Food;
}

fn setup_screen(world: &mut World, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>)
{
    write!(stdout, "{}", termion::clear::All).unwrap();

    let size = termion::terminal_size().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(size.0 / 2 - (world.width/2) as u16, size.1 / 2 - (world.height/2) as u16)).unwrap();
    write!(stdout, "{}", termion::cursor::Save).unwrap();
}

fn draw(world: &mut World, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>)
{
    let mut this_frame_screen = world.screen.clone();

    for segment in world.snake.segments.iter().take(world.snake.segments.len() - 1)
    {
        this_frame_screen[(segment.x + segment.y * world.width as i32) as usize] = Tile::Snake;
    }
    
    {
        let segment: Point = (*world.snake.segments.last().unwrap()).clone();
        this_frame_screen[(segment.x + segment.y * world.width as i32) as usize] = Tile::SnakeHead;
    }

    let snake_head_char = match world.snake.dir
    {
        Dir::Up => '^',
        Dir::Right => '>',
        Dir::Down => 'v',
        Dir::Left => '<'
    };

    write!(stdout, "{}", termion::cursor::Restore).unwrap();
    write!(stdout, "{}", termion::cursor::Save).unwrap();

    write!(stdout, "{}", termion::cursor::Up(1)).unwrap();
    write!(stdout, "Score: {}", world.score).unwrap();
    
    write!(stdout, "{}", termion::cursor::Restore).unwrap();
    write!(stdout, "{}", termion::cursor::Save).unwrap();

    for y in 0..world.height
    {
        write!(stdout, "{}", termion::cursor::Restore).unwrap();
        write!(stdout, "{}", termion::cursor::Save).unwrap();
        write!(stdout, "{}", termion::cursor::Down((y+1) as u16)).unwrap();


        for x in 0..world.width
        {
            match this_frame_screen[x+y*world.width]
            {
                Tile::Floor => write!(stdout, " "),
                Tile::Wall => write!(stdout, "#"),
                Tile::Food => write!(stdout, "@"),
                Tile::Snake => write!(stdout, "*"),
                Tile::SnakeHead => write!(stdout, "{}", snake_head_char)
            }.unwrap();
        }
    }
    
    stdout.flush().unwrap();
}

fn main() 
{
    let mut world = init_world(36, 20, 3);

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().events();

    setup_screen(&mut world, &mut stdout);
    
    loop
    {
        let mut need_place_food = false;

        {
            let snake = &mut world.snake;
            let screen = &mut world.screen;

            let mut head: Point = (*snake.segments.last().unwrap()).clone();

            match snake.dir
            {
                Dir::Up     => head.y -= 1,
                Dir::Right  => head.x += 1,
                Dir::Down   => head.y += 1,
                Dir::Left   => head.x -= 1
            };
            
            let head_tile = screen[(head.x + head.y * world.width as i32) as usize].clone();

            let mut hit_self = false;

            for segment in snake.segments.iter().take(snake.segments.len() - 1)
            {
                if segment == &head
                {
                    hit_self = true;
                }
            }

            if head_tile == Tile::Wall || hit_self
            {
                break;
            }
            else if head_tile == Tile::Food
            {
                screen[(head.x + head.y * world.width as i32) as usize] = Tile::Floor;
                need_place_food = true;
                world.score += 1;
            }
            
            if !need_place_food
            {
                snake.segments.remove(0);
            }

            snake.segments.push(head.clone());

            let event = stdin.next();

            match event
            {
                None => {},
                Some(x) => match x
                {
                    Ok(y) => match y
                    {
                        Event::Key(Key::Char('q')) => break,

                        Event::Key(Key::Up)     => if snake.dir != Dir::Down  { snake.dir = Dir::Up },
                        Event::Key(Key::Right)  => if snake.dir != Dir::Left  { snake.dir = Dir::Right },
                        Event::Key(Key::Down)   => if snake.dir != Dir::Up    { snake.dir = Dir::Down },
                        Event::Key(Key::Left)   => if snake.dir != Dir::Right { snake.dir = Dir::Left },
                        _ => {}
                    }
                    _ => {}
                }
            }

            thread::sleep(Duration::from_millis(80));
        }

        if need_place_food
        {
            place_food(&mut world);
        }

        draw(&mut world, &mut stdout);
    }
    
    write!(stdout, "\n\n{}", termion::cursor::Left(9999)).unwrap();
}
