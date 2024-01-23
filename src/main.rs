use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, MeshBuilder, DrawParam, DrawMode, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use nalgebra as na;

const SNAKE_SIZE: f32 = 20.0;
const MOVE_TIME: f32 = 0.1; // Time in seconds between moves

struct GameState {
    snake_pos: na::Point2<f32>,
    velocity: na::Vector2<f32>,
    last_update: f32, // Time since last update
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        let s = GameState {
            snake_pos: na::Point2::new(200.0, 200.0),
            velocity: na::Vector2::new(SNAKE_SIZE, 0.0),
            last_update: 0.0,
        };
        Ok(s)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.last_update += ctx.time.delta().as_secs_f32();
        if self.last_update >= MOVE_TIME {
            self.last_update = 0.0;
            self.snake_pos += self.velocity;

            // Handle keyboard input
            if ctx.keyboard.is_key_pressed(KeyCode::Right) {
                self.velocity = na::Vector2::new(SNAKE_SIZE, 0.0);
            } else if ctx.keyboard.is_key_pressed(KeyCode::Left) {
                self.velocity = na::Vector2::new(-SNAKE_SIZE, 0.0);
            } else if ctx.keyboard.is_key_pressed(KeyCode::Up) {
                self.velocity = na::Vector2::new(0.0, -SNAKE_SIZE);
            } else if ctx.keyboard.is_key_pressed(KeyCode::Down) {
                self.velocity = na::Vector2::new(0.0, SNAKE_SIZE);
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Some(Color::from_rgb(230, 230, 230)));

        // Create the MeshBuilder and build the mesh
        let mut mesh_builder = MeshBuilder::new();
        mesh_builder.rectangle(
            DrawMode::fill(),
            Rect::new(self.snake_pos.x, self.snake_pos.y, SNAKE_SIZE, SNAKE_SIZE),
            Color::from_rgb(50, 150, 50),
        )?;

        // Build the mesh from the MeshBuilder
        let mesh_data = mesh_builder.build();
        let snake_mesh = graphics::Mesh::from_data(ctx, mesh_data);
        canvas.draw(&snake_mesh, DrawParam::default());

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
    let cb = ggez::ContextBuilder::new("snake_game", "Jacob Mish");
    let (mut ctx, event_loop) = cb.build()?;  // Add 'mut' to make ctx mutable
    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

