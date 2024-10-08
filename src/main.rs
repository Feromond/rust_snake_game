#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{mint, Context, GameResult};
use nalgebra::{self as na};
use rand::Rng;

const REFERENCE_WIDTH: f32 = 1400.0;
const REFERENCE_HEIGHT: f32 = 1050.0;
const REFERENCE_SNAKE_SIZE: f32 = 50.0;

// Controls snake speed.
const EASY_MOVE_TIME: f32 = 0.12;
const NORMAL_MOVE_TIME: f32 = 0.08;
const HARD_MOVE_TIME: f32 = 0.065;

// Special mode constants
const SPECIAL_START_MOVE_TIME: f32 = 0.15; // Initial slower speed for Special
const SPEED_UP_FACTOR: f32 = 0.95; // Multiplier for the speed increase
const MIN_MOVE_TIME: f32 = 0.03; // Minimum move time to avoid it being too fast

// Helps remove floating point errors
const EPSILON: f32 = 0.01;

enum GameMode {
    Menu,
    Playing,
}

enum Difficulty {
    Easy,
    Normal,
    Hard,
    Special,
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
    boundary_width: f32,
    boundary_height: f32,
    scaled_snake_size: f32,
    offset_x: f32, // Offset to center the game in case of extra window space
    offset_y: f32,
    difficulty: Difficulty,
    move_time: f32, // Store the current move time based on difficulty
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let (window_width, window_height) = GameState::get_window_size(ctx);
        let (boundary_width, boundary_height) =
            GameState::calculate_locked_boundary(window_width, window_height);
        let scale = boundary_width / REFERENCE_WIDTH;
        let scaled_snake_size = REFERENCE_SNAKE_SIZE * scale;
        let offset_x = (window_width - boundary_width) / 2.0;
        let offset_y = (window_height - boundary_height) / 2.0;

        let s = GameState {
            snake_body: vec![SnakeSegment {
                pos: na::Point2::new(
                    ((boundary_width / 4.0) / scaled_snake_size).floor() * scaled_snake_size,
                    ((boundary_height / 4.0) / scaled_snake_size).floor() * scaled_snake_size,
                ),
            }],
            food: Food {
                pos: GameState::get_random_food_position(boundary_width, boundary_height, scale),
            },
            velocity: na::Vector2::new(scaled_snake_size, 0.0),
            last_update: 0.0,
            score: 0,
            high_score: 0,
            mode: GameMode::Menu,
            window_width,
            window_height,
            scale,
            boundary_width,
            boundary_height,
            scaled_snake_size,
            offset_x,
            offset_y,
            difficulty: Difficulty::Normal, // Set default difficulty to Normal
            move_time: NORMAL_MOVE_TIME,    // Set default move time to Normal speed
        };
        Ok(s)
    }

    fn calculate_locked_boundary(window_width: f32, window_height: f32) -> (f32, f32) {
        // Lock the boundary to a 4:3 aspect ratio
        let aspect_ratio = 4.0 / 3.0;
        let boundary_width;
        let boundary_height;

        if window_width / window_height > aspect_ratio {
            // If the window is wider than 4:3, lock the height and calculate the width
            boundary_height = window_height;
            boundary_width = boundary_height * aspect_ratio;
        } else {
            // If the window is taller than 4:3, lock the width and calculate the height
            boundary_width = window_width;
            boundary_height = boundary_width / aspect_ratio;
        }

        (boundary_width, boundary_height)
    }

    fn get_random_food_position(
        boundary_width: f32,
        boundary_height: f32,
        scale: f32,
    ) -> na::Point2<f32> {
        let mut rng = rand::thread_rng();
        na::Point2::new(
            (rng.gen_range(0..(boundary_width as u32 / (REFERENCE_SNAKE_SIZE * scale) as u32))
                as f32)
                * (REFERENCE_SNAKE_SIZE * scale),
            (rng.gen_range(0..(boundary_height as u32 / (REFERENCE_SNAKE_SIZE * scale) as u32))
                as f32)
                * (REFERENCE_SNAKE_SIZE * scale),
        )
    }

    fn reset_game_state(&mut self) {
        self.snake_body = vec![SnakeSegment {
            pos: na::Point2::new(200.0 * self.scale, 200.0 * self.scale),
        }];
        self.food.pos = GameState::get_random_food_position(
            self.boundary_width,
            self.boundary_height,
            self.scale,
        );
        self.velocity = na::Vector2::new(self.scaled_snake_size, 0.0);
        self.score = 0;

        // Set move_time based on the selected difficulty
        self.move_time = match self.difficulty {
            Difficulty::Easy => EASY_MOVE_TIME,
            Difficulty::Normal => NORMAL_MOVE_TIME,
            Difficulty::Hard => HARD_MOVE_TIME,
            Difficulty::Special => SPECIAL_START_MOVE_TIME,
        };
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
        let (boundary_width, boundary_height) =
            GameState::calculate_locked_boundary(window_width, window_height);
        self.boundary_width = boundary_width;
        self.boundary_height = boundary_height;
        self.scale = boundary_width / REFERENCE_WIDTH;
        self.scaled_snake_size = REFERENCE_SNAKE_SIZE * self.scale;
        self.offset_x = (window_width - boundary_width) / 2.0;
        self.offset_y = (window_height - boundary_height) / 2.0;
    }

    fn scaled_rect(&self, pos: na::Point2<f32>) -> Rect {
        Rect::new(
            self.offset_x + pos.x,
            self.offset_y + pos.y,
            self.scaled_snake_size,
            self.scaled_snake_size,
        )
    }

    fn draw_border(&self, mesh_builder: &mut MeshBuilder) {
        let border_thickness = 5.0 * self.scale;
        let rect = Rect::new(
            self.offset_x,
            self.offset_y,
            self.boundary_width,
            self.boundary_height,
        );
        let _ = mesh_builder.rectangle(
            DrawMode::stroke(border_thickness),
            rect,
            Color::from_rgb(255, 0, 0),
        );
    }

    fn check_border_collisions(&self) -> bool {
        let head_pos = self.snake_body[0].pos;

        if head_pos.x < 0.0 - EPSILON
            || head_pos.y < 0.0 - EPSILON
            || head_pos.x + self.scaled_snake_size > self.boundary_width + EPSILON
            || head_pos.y + self.scaled_snake_size > self.boundary_height + EPSILON
        {
            return true;
        }
        false
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
                if self.last_update >= self.move_time {
                    self.last_update = 0.0;
                    // Clone the position of the last segment
                    let last_pos = self.snake_body.last().unwrap().pos;
                    // Movement for snake
                    for i in (1..self.snake_body.len()).rev() {
                        self.snake_body[i].pos = self.snake_body[i - 1].pos;
                    }
                    self.snake_body[0].pos += self.velocity;

                    // Check if snake ate the food with adjusted collision detection
                    let dist_x = (self.snake_body[0].pos.x - self.food.pos.x).abs();
                    let dist_y = (self.snake_body[0].pos.y - self.food.pos.y).abs();
                    if dist_x < self.scaled_snake_size - EPSILON
                        && dist_y < self.scaled_snake_size - EPSILON
                    {
                        // Eat the food and grow
                        self.snake_body.push(SnakeSegment { pos: last_pos });
                        self.score += 1;

                        // If in Special difficulty, increase speed
                        if let Difficulty::Special = self.difficulty {
                            // Reduce the move time by the speed-up factor
                            self.move_time *= SPEED_UP_FACTOR;
                            // Ensure move_time doesn't go below a certain minimum
                            if self.move_time < MIN_MOVE_TIME {
                                self.move_time = MIN_MOVE_TIME;
                            }
                        }

                        // Generate new food position and ensure it doesn't overlap with the snake
                        loop {
                            self.food.pos = GameState::get_random_food_position(
                                self.boundary_width,
                                self.boundary_height,
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

                    // Check for boundary collisions
                    if self.check_border_collisions() {
                        self.mode = GameMode::Menu;
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Some(Color::BLACK));
        let mut mesh_builder = MeshBuilder::new();

        match self.mode {
            GameMode::Menu => {
                let mut menu_text = Text::new(format!("Press Enter to Start\n     ESC to Exit\n"));
                menu_text.set_scale(graphics::PxScale::from(60.0 * self.scale));
                canvas.draw(
                    &menu_text,
                    DrawParam::default().dest(mint::Point2 {
                        x: self.boundary_width * 0.5 - (350.0 * self.scale) + self.offset_x,
                        y: self.boundary_height * 0.35 + self.offset_y,
                    }),
                );

                // Draw difficulty text separately for each difficulty level with different colors
                let easy_color = if let Difficulty::Easy = self.difficulty {
                    Color::from_rgb(0, 255, 0) // Green for selected
                } else {
                    Color::from_rgb(180, 255, 200) // Light green for unselected
                };
                let normal_color = if let Difficulty::Normal = self.difficulty {
                    Color::from_rgb(255, 255, 0) // Yellow for selected
                } else {
                    Color::from_rgb(255, 255, 200) // Light yellow for unselected
                };
                let hard_color = if let Difficulty::Hard = self.difficulty {
                    Color::from_rgb(255, 0, 0) // Red for selected
                } else {
                    Color::from_rgb(255, 200, 200) // Light red for unselected
                };
                let special_color = if let Difficulty::Special = self.difficulty {
                    Color::from_rgb(255, 100, 255) // Purple for selected
                } else {
                    Color::from_rgb(255, 200, 255) // Light purple for unselected
                };

                let mut easy_text = Text::new("1: Easy");
                easy_text.set_scale(graphics::PxScale::from(50.0 * self.scale));
                canvas.draw(
                    &easy_text,
                    DrawParam::default()
                        .dest(mint::Point2 {
                            x: self.boundary_width * 0.5 - (350.0 * self.scale) + self.offset_x,
                            y: self.boundary_height * 0.50 + self.offset_y,
                        })
                        .color(easy_color),
                );

                let mut normal_text = Text::new("2: Normal");
                normal_text.set_scale(graphics::PxScale::from(50.0 * self.scale));
                canvas.draw(
                    &normal_text,
                    DrawParam::default()
                        .dest(mint::Point2 {
                            x: self.boundary_width * 0.67 - (350.0 * self.scale) + self.offset_x,
                            y: self.boundary_height * 0.50 + self.offset_y,
                        })
                        .color(normal_color),
                );

                let mut hard_text = Text::new("3: Hard");
                hard_text.set_scale(graphics::PxScale::from(50.0 * self.scale));
                canvas.draw(
                    &hard_text,
                    DrawParam::default()
                        .dest(mint::Point2 {
                            x: self.boundary_width * 0.88 - (350.0 * self.scale) + self.offset_x,
                            y: self.boundary_height * 0.50 + self.offset_y,
                        })
                        .color(hard_color),
                );

                let mut special_text = Text::new("4: Special");
                special_text.set_scale(graphics::PxScale::from(50.0 * self.scale));
                canvas.draw(
                    &special_text,
                    DrawParam::default()
                        .dest(mint::Point2 {
                            x: self.boundary_width * 0.65 - (350.0 * self.scale) + self.offset_x,
                            y: self.boundary_height * 0.57 + self.offset_y,
                        })
                        .color(special_color),
                );

                let mut menu_high_score_text =
                    Text::new(format!("High Score: {}", self.high_score));
                menu_high_score_text.set_scale(graphics::PxScale::from(60.0 * self.scale));
                canvas.draw(
                    &menu_high_score_text,
                    DrawParam::default()
                        .dest(mint::Point2 {
                            x: self.boundary_width * 0.61 - (350.0 * self.scale) + self.offset_x,
                            y: self.boundary_height * 0.65 + self.offset_y,
                        })
                        .color(Color::from_rgb(0, 255, 0)),
                )
            }
            GameMode::Playing => {
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

                // Draw the game boundary (red border)
                self.draw_border(&mut mesh_builder);

                let mesh = Mesh::from_data(ctx, mesh_builder.build());
                canvas.draw(&mesh, DrawParam::default());

                // Draw score
                let mut score_text = Text::new(format!("Score: {}", self.score));
                score_text.set_scale(graphics::PxScale::from(40.0 * self.scale));
                canvas.draw(
                    &score_text,
                    DrawParam::default().dest(mint::Point2 {
                        x: (10.0 * self.scale) + self.offset_x,
                        y: (60.0 * self.scale) + self.offset_y,
                    }),
                );
                let mut high_score_text = Text::new(format!("High Score: {}", self.high_score));
                high_score_text.set_scale(graphics::PxScale::from(40.0 * self.scale));
                canvas.draw(
                    &high_score_text,
                    DrawParam::default()
                        .dest(mint::Point2 {
                            x: (10.0 * self.scale) + self.offset_x,
                            y: (10.0 * self.scale) + self.offset_y,
                        })
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
                } else if let Some(KeyCode::Key1) = key.keycode {
                    // Set difficulty to Easy
                    self.difficulty = Difficulty::Easy;
                } else if let Some(KeyCode::Key2) = key.keycode {
                    // Set difficulty to Normal
                    self.difficulty = Difficulty::Normal;
                } else if let Some(KeyCode::Key3) = key.keycode {
                    // Set difficulty to Hard
                    self.difficulty = Difficulty::Hard;
                } else if let Some(KeyCode::Key4) = key.keycode {
                    // Set difficulty to Special
                    self.difficulty = Difficulty::Special;
                }
            }
            GameMode::Playing => match key.keycode {
                Some(KeyCode::Right) | Some(KeyCode::D) if self.velocity.x == 0.0 => {
                    self.velocity = na::Vector2::new(self.scaled_snake_size, 0.0);
                }
                Some(KeyCode::Left) | Some(KeyCode::A) if self.velocity.x == 0.0 => {
                    self.velocity = na::Vector2::new(-self.scaled_snake_size, 0.0);
                }
                Some(KeyCode::Up) | Some(KeyCode::W) if self.velocity.y == 0.0 => {
                    self.velocity = na::Vector2::new(0.0, -self.scaled_snake_size);
                }
                Some(KeyCode::Down) | Some(KeyCode::S) if self.velocity.y == 0.0 => {
                    self.velocity = na::Vector2::new(0.0, self.scaled_snake_size);
                }
                Some(KeyCode::Escape) => {
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
                .min_dimensions(800.0, 600.0)
                .resizable(true)
                .transparent(true),
        )
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
