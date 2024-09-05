use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{mint, Context, GameResult};
use nalgebra as na;
use rand::Rng;

const REFERENCE_WIDTH: f32 = 1200.0;
const REFERENCE_HEIGHT: f32 = 800.0;
const REFERENCE_SNAKE_SIZE: f32 = 50.0;
const MOVE_TIME: f32 = 0.075; // Time in seconds between moves

enum GameMode {
    Menu,
    Playing,
}

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
    last_update: f32,
    score: i32,
    high_score: i32,
    mode: GameMode,
    window_width: f32,
    window_height: f32,
    scale: f32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let (window_width, window_height) = GameState::get_window_size(ctx);
        let scale = GameState::calculate_scale(window_width, window_height);
        let s = GameState {
            snake_body: vec![SnakeSegment {
                pos: na::Point2::new(200.0 * scale, 200.0 * scale),
            }],
            food: Food {
                pos: GameState::get_random_food_position(window_width, window_height, scale),
            },
            velocity: na::Vector2::new(REFERENCE_SNAKE_SIZE * scale, 0.0),
            last_update: 0.0,
            score: 0,
            high_score: 0,
            mode: GameMode::Menu,
            window_width,
            window_height,
            scale,
        };
        Ok(s)
    }

    fn calculate_scale(window_width: f32, window_height: f32) -> f32 {
        // Use the smaller scale factor to maintain consistent aspect ratio
        let scale_width = window_width / REFERENCE_WIDTH;
        let scale_height = window_height / REFERENCE_HEIGHT;
        scale_width.min(scale_height)
    }

    fn get_random_food_position(
        window_width: f32,
        window_height: f32,
        scale: f32,
    ) -> na::Point2<f32> {
        let mut rng = rand::thread_rng();
        na::Point2::new(
            (rng.gen_range(0..(window_width as u32 / (REFERENCE_SNAKE_SIZE * scale) as u32))
                as f32)
                * (REFERENCE_SNAKE_SIZE * scale),
            (rng.gen_range(0..(window_height as u32 / (REFERENCE_SNAKE_SIZE * scale) as u32))
                as f32)
                * (REFERENCE_SNAKE_SIZE * scale),
        )
    }

    fn reset_game_state(&mut self) {
        self.snake_body = vec![SnakeSegment {
            pos: na::Point2::new(200.0 * self.scale, 200.0 * self.scale),
        }];
        self.food.pos =
            GameState::get_random_food_position(self.window_width, self.window_height, self.scale);
        self.velocity = na::Vector2::new(REFERENCE_SNAKE_SIZE * self.scale, 0.0);
        self.score = 0;
    }

    fn get_window_size(ctx: &mut Context) -> (f32, f32) {
        let canvas = Canvas::from_frame(ctx, Some(Color::BLACK));
        let rect = canvas.screen_coordinates().unwrap();
        (rect.w, rect.h)
    }

    fn handle_resize(&mut self, ctx: &mut Context) {
        let (window_width, window_height) = GameState::get_window_size(ctx);
        self.window_width = window_width;
        self.window_height = window_height;
        self.scale = GameState::calculate_scale(window_width, window_height);
    }

    fn scaled_rect(&self, pos: na::Point2<f32>) -> Rect {
        Rect::new(
            pos.x,
            pos.y,
            REFERENCE_SNAKE_SIZE * self.scale,
            REFERENCE_SNAKE_SIZE * self.scale,
        )
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_resize(ctx);
        match self.mode {
            GameMode::Menu => {
                if self.score > self.high_score {
                    self.high_score = self.score;
                }
                // No game updates in menu mode (I can consider placing some animations here maybe??)
            }
            GameMode::Playing => {
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

                    // Check if snake ate the food
                    if self.snake_body[0]
                        .pos
                        .coords
                        .zip_map(&self.food.pos.coords, |a, b| (a - b).abs())
                        .iter()
                        .sum::<f32>()
                        < REFERENCE_SNAKE_SIZE * self.scale
                    {
                        // Eat the food and grow
                        self.snake_body.push(SnakeSegment { pos: last_pos });
                        self.score += 1; // Increase the score

                        // Generate new food position and ensure it doesn't overlap with the snake
                        loop {
                            self.food.pos = GameState::get_random_food_position(
                                self.window_width,
                                self.window_height,
                                self.scale,
                            );
                            if !self
                                .snake_body
                                .iter()
                                .any(|segment| segment.pos == self.food.pos)
                            {
                                break;
                            }
                        }
                    }

                    // Check for collisions with self
                    for segment in &self.snake_body[1..] {
                        if self.snake_body[0].pos == segment.pos {
                            self.mode = GameMode::Menu;
                            break;
                        }
                    }

                    // Check for out-of-bounds
                    let head_pos = self.snake_body[0].pos;
                    if head_pos.x < 0.0
                        || head_pos.y < 0.0
                        || head_pos.x + REFERENCE_SNAKE_SIZE * self.scale > self.window_width
                        || head_pos.y + REFERENCE_SNAKE_SIZE * self.scale > self.window_height
                    {
                        self.mode = GameMode::Menu;
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Some(Color::BLACK));

        match self.mode {
            GameMode::Menu => {
                let mut menu_text = Text::new(format!("Press Enter to Start\nESC to Exit\n"));
                menu_text.set_scale(graphics::PxScale::from(60.0));
                canvas.draw(
                    &menu_text,
                    DrawParam::default().dest(mint::Point2 {
                        x: self.window_width * 0.5 - 350.0,
                        y: self.window_height * 0.4,
                    }),
                );
                let mut menu_high_score_text =
                    Text::new(format!("High Score: {}", self.high_score));
                menu_high_score_text.set_scale(graphics::PxScale::from(60.0));
                canvas.draw(
                    &menu_high_score_text,
                    DrawParam::default()
                        .dest(mint::Point2 {
                            x: self.window_width * 0.5 - 350.0,
                            y: self.window_height * 0.55,
                        })
                        .color(Color::from_rgb(0, 255, 0)),
                )
            }
            GameMode::Playing => {
                let mut mesh_builder = MeshBuilder::new();

                // Draw the snake
                for segment in &self.snake_body {
                    mesh_builder.rectangle(
                        DrawMode::fill(),
                        self.scaled_rect(segment.pos),
                        Color::from_rgb(50, 150, 50),
                    )?;
                }

                // Draw the food
                mesh_builder.rectangle(
                    DrawMode::fill(),
                    self.scaled_rect(self.food.pos),
                    Color::from_rgb(255, 0, 0),
                )?;

                let mesh = Mesh::from_data(ctx, mesh_builder.build());
                canvas.draw(&mesh, DrawParam::default());

                // Draw score
                let mut score_text = Text::new(format!("Score: {}", self.score));
                score_text.set_scale(graphics::PxScale::from(40.0));
                canvas.draw(
                    &score_text,
                    DrawParam::default().dest(mint::Point2 { x: 10.0, y: 60.0 }),
                );
                let mut high_score_text = Text::new(format!("High Score: {}", self.high_score));
                high_score_text.set_scale(graphics::PxScale::from(40.0));
                canvas.draw(
                    &high_score_text,
                    DrawParam::default()
                        .dest(mint::Point2 { x: 10.0, y: 10.0 })
                        .color(Color::from_rgb(0, 255, 0)),
                );
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyInput, _repeat: bool) -> GameResult {
        match self.mode {
            GameMode::Menu => {
                if let Some(KeyCode::Return) = key.keycode {
                    // Start the game
                    self.mode = GameMode::Playing;
                    self.reset_game_state();
                } else if let Some(KeyCode::Escape) = key.keycode {
                    ctx.request_quit();
                }
            }
            GameMode::Playing => match key.keycode {
                Some(KeyCode::Right) | Some(KeyCode::D) if self.velocity.x == 0.0 => {
                    self.velocity = na::Vector2::new(REFERENCE_SNAKE_SIZE * self.scale, 0.0);
                }
                Some(KeyCode::Left) | Some(KeyCode::A) if self.velocity.x == 0.0 => {
                    self.velocity = na::Vector2::new(-REFERENCE_SNAKE_SIZE * self.scale, 0.0);
                }
                Some(KeyCode::Up) | Some(KeyCode::W) if self.velocity.y == 0.0 => {
                    self.velocity = na::Vector2::new(0.0, -REFERENCE_SNAKE_SIZE * self.scale);
                }
                Some(KeyCode::Down) | Some(KeyCode::S) if self.velocity.y == 0.0 => {
                    self.velocity = na::Vector2::new(0.0, REFERENCE_SNAKE_SIZE * self.scale);
                }
                Some(KeyCode::Escape) => {
                    // Return to menu
                    self.mode = GameMode::Menu;
                }
                _ => {}
            },
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Rust Snake Game", "Jacob Mish")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Game"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(REFERENCE_WIDTH, REFERENCE_HEIGHT)
                .min_dimensions(REFERENCE_WIDTH, REFERENCE_HEIGHT)
                .resizable(true),
        )
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
