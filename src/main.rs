use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, MeshBuilder, DrawParam, DrawMode, Rect, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use nalgebra as na;
use ggez::mint;
use rand::Rng; // for generating random numbers

const SNAKE_SIZE: f32 = 50.0;
const MOVE_TIME: f32 = 0.075; // Time in seconds between moves
const WINDOW_WIDTH: f32 = 1600.0; // Set the window width
const WINDOW_HEIGHT: f32 = 1200.0; // Set the window height

struct Food {
    pos: na::Point2<f32>,
}

struct SnakeSegment {
    pos: na::Point2<f32>,
}

struct GameState {
    snake_body: Vec<SnakeSegment>,
    food: Food,
    velocity: na::Vector2<f32>,
    last_update: f32, // Time since last update
    score: i32,       // Track the score
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        let s = GameState {
            snake_body: vec![SnakeSegment { pos: na::Point2::new(200.0, 200.0) }],
            food: Food { pos: GameState::get_random_food_position() },
            velocity: na::Vector2::new(SNAKE_SIZE, 0.0),
            last_update: 0.0,
            score: 0,
        };
        Ok(s)
    }

    fn get_random_food_position() -> na::Point2<f32> {
        let mut rng = rand::thread_rng();
        na::Point2::new(
            (rng.gen_range(0..(WINDOW_WIDTH as u32 / SNAKE_SIZE as u32)) as f32) * SNAKE_SIZE,
            (rng.gen_range(0..(WINDOW_HEIGHT as u32 / SNAKE_SIZE as u32)) as f32) * SNAKE_SIZE,
        )
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.last_update += ctx.time.delta().as_secs_f32();
        if self.last_update >= MOVE_TIME {
            self.last_update = 0.0;
    
            // Clone the position of the last segment
            let last_pos = self.snake_body.last().unwrap().pos;
    
            // Movement for snake
            for i in (1..self.snake_body.len()).rev() {
                self.snake_body[i].pos = self.snake_body[i - 1].pos;
            }
            self.snake_body[0].pos += self.velocity;
    
            // Collision detection with food
            if self.snake_body[0].pos == self.food.pos {
                // Eat the food and grow
                self.snake_body.push(SnakeSegment { pos: last_pos });
                self.score += 1;  // Increase the score

                // Ensure the new food does not spawn on the snake
                loop {
                    self.food.pos = GameState::get_random_food_position();
                    if !self.snake_body.iter().any(|segment| segment.pos == self.food.pos) {
                        break;
                    }
                }
            }

            // Self-collision detection
            for segment in &self.snake_body[1..] {
                if segment.pos == self.snake_body[0].pos {
                    ctx.request_quit();  // End the game if the snake collides with itself
                }
            }

            // Check if the snake is within the window bounds after moving
            let head_pos = self.snake_body[0].pos;
            if head_pos.x < 0.0 || head_pos.y < 0.0 || head_pos.x >= WINDOW_WIDTH || head_pos.y >= WINDOW_HEIGHT {
                ctx.request_quit();  // End the game if the snake goes out of bounds
            }

            // Handle keyboard input
            if ctx.keyboard.is_key_pressed(KeyCode::Right) && self.velocity.x == 0.0 {
                self.velocity = na::Vector2::new(SNAKE_SIZE, 0.0);
            } else if ctx.keyboard.is_key_pressed(KeyCode::Left) && self.velocity.x == 0.0 {
                self.velocity = na::Vector2::new(-SNAKE_SIZE, 0.0);
            } else if ctx.keyboard.is_key_pressed(KeyCode::Up) && self.velocity.y == 0.0 {
                self.velocity = na::Vector2::new(0.0, -SNAKE_SIZE);
            } else if ctx.keyboard.is_key_pressed(KeyCode::Down) && self.velocity.y == 0.0 {
                self.velocity = na::Vector2::new(0.0, SNAKE_SIZE);
            }
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Some(Color::from_rgb(230, 230, 230)));
        // Create the MeshBuilder and build the mesh
        let mut mesh_builder = MeshBuilder::new();

        // Draw each snake segment
        for segment in &self.snake_body {
            mesh_builder.rectangle(
                DrawMode::fill(),
                Rect::new(segment.pos.x, segment.pos.y, SNAKE_SIZE, SNAKE_SIZE),
                Color::from_rgb(50, 150, 50),
            )?;
        }

        // Draw food
        mesh_builder.rectangle(
            DrawMode::fill(),
            Rect::new(self.food.pos.x, self.food.pos.y, SNAKE_SIZE, SNAKE_SIZE),
            Color::from_rgb(255, 0, 0), // Red color for food
        )?;

        // Build the mesh from the MeshBuilder
        let mesh_data = mesh_builder.build();
        let snake_mesh = graphics::Mesh::from_data(ctx, mesh_data);
        canvas.draw(&snake_mesh, DrawParam::default());

        // Create a Text object with the score
        let score_str = format!("Score: {}", self.score);

        let score_fragment = graphics::TextFragment::new(score_str)
            .scale(graphics::PxScale::from(40.0))  // Set the size of the text
            .color(graphics::Color::from_rgb(255, 0, 0));  // Set the color of the text


        let score_text = graphics::Text::new(score_fragment);

        let score_position = na::Point2::new(WINDOW_WIDTH / 2.0 - 50.0, 20.0);
        
        // Convert na::Point2<f32> to mint::Point2<f32> for the dest method
        let score_position_mint = mint::Point2 { x: score_position.x, y: score_position.y };

        canvas.draw(&score_text, DrawParam::default().dest(score_position_mint));

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyInput, _repeat: bool) -> GameResult {
        if key.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("snake_game", "Jacob Mish")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
