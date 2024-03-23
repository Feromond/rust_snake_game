use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{mint, Context, GameResult};
use nalgebra as na;
use rand::Rng;

const SNAKE_SIZE: f32 = 50.0;
const MOVE_TIME: f32 = 0.075; // Time in seconds between moves
const WINDOW_WIDTH: f32 = 1600.0; // Set the window width
const WINDOW_HEIGHT: f32 = 1000.0; // Set the window height

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
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        let s = GameState {
            snake_body: vec![SnakeSegment {
                pos: na::Point2::new(200.0, 200.0),
            }],
            food: Food {
                pos: GameState::get_random_food_position(),
            },
            velocity: na::Vector2::new(SNAKE_SIZE, 0.0),
            last_update: 0.0,
            score: 0,
            high_score: 0,
            mode: GameMode::Menu,
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

    fn reset_game_state(&mut self) {
        self.snake_body = vec![SnakeSegment {
            pos: na::Point2::new(200.0, 200.0),
        }];
        self.food.pos = GameState::get_random_food_position();
        self.velocity = na::Vector2::new(SNAKE_SIZE, 0.0);
        self.score = 0;
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
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

                    // Collision detection with food
                    if self.snake_body[0].pos == self.food.pos {
                        // Eat the food and grow
                        self.snake_body.push(SnakeSegment { pos: last_pos });
                        self.score += 1; // Increase the score

                        // Ensure the new food does not spawn on the snake
                        loop {
                            self.food.pos = GameState::get_random_food_position();
                            if !self
                                .snake_body
                                .iter()
                                .any(|segment| segment.pos == self.food.pos)
                            {
                                break;
                            }
                        }
                    }

                    for segment in &self.snake_body[1..] {
                        if segment.pos == self.snake_body[0].pos {
                            self.mode = GameMode::Menu;
                            break;
                        }
                    }

                    let head_pos = self.snake_body[0].pos;
                    if head_pos.x < 0.0
                        || head_pos.y < 0.0
                        || head_pos.x >= WINDOW_WIDTH
                        || head_pos.y >= WINDOW_HEIGHT
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
                        x: WINDOW_WIDTH * 0.5 - 350.0,
                        y: WINDOW_HEIGHT * 0.4,
                    }),
                );
                let mut menu_high_score_text =
                    Text::new(format!("High Score: {}", self.high_score));
                menu_high_score_text.set_scale(graphics::PxScale::from(60.0));
                canvas.draw(
                    &menu_high_score_text,
                    DrawParam::default()
                        .dest(mint::Point2 {
                            x: WINDOW_WIDTH * 0.5 - 350.0,
                            y: WINDOW_HEIGHT * 0.55,
                        })
                        .color(Color::from_rgb(0, 255, 0)),
                )
            }
            GameMode::Playing => {
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
                    Color::from_rgb(255, 0, 0),
                )?;

                let mesh = Mesh::from_data(ctx, mesh_builder.build());
                canvas.draw(&mesh, DrawParam::default());

                // Create a Text object with the score
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
            GameMode::Playing => {
                match key.keycode {
                    Some(KeyCode::Right) | Some(KeyCode::D) if self.velocity.x == 0.0 => {
                        self.velocity = na::Vector2::new(SNAKE_SIZE, 0.0);
                    }
                    Some(KeyCode::Left) | Some(KeyCode::A) if self.velocity.x == 0.0 => {
                        self.velocity = na::Vector2::new(-SNAKE_SIZE, 0.0);
                    }
                    Some(KeyCode::Up) | Some(KeyCode::W) if self.velocity.y == 0.0 => {
                        self.velocity = na::Vector2::new(0.0, -SNAKE_SIZE);
                    }
                    Some(KeyCode::Down) | Some(KeyCode::S) if self.velocity.y == 0.0 => {
                        self.velocity = na::Vector2::new(0.0, SNAKE_SIZE);
                    }
                    Some(KeyCode::Escape) => {
                        // Return to menu
                        self.mode = GameMode::Menu;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Rust Snake Game", "Jacob Mish")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
