use crate::constants::*;
use crate::game_state::GameState;
use crate::types::*;
use ggez::{
    audio::SoundSource,
    event::EventHandler,
    graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Text},
    input::keyboard::{KeyCode, KeyInput},
    mint,
    Context,
    GameResult,
};
use nalgebra as na;

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_resize(ctx);
        match self.mode {
            GameMode::Menu => {
                if self.score > self.high_score {
                    self.high_score = self.score;
                }
                // Stop game music if it's playing when returning to menu
                if self.game_music.playing() {
                    self.game_music.set_pitch(1.0); // Reset pitch before stopping
                    self.game_music.stop(ctx)?;
                }
                // Stop special music if it's playing
                if self.special_mode_music.playing() {
                    self.special_mode_music.stop(ctx)?;
                }
                // Start menu music if it's not playing
                if !self.menu_music.playing() {
                    self.menu_music.play(ctx)?;
                }
            }
            GameMode::Playing => {
                // Stop menu music if it's playing when starting the game
                if self.menu_music.playing() {
                    self.menu_music.stop(ctx)?;
                }
                // Start appropriate game music if not playing
                if !self.game_music.playing() && !self.special_mode_music.playing() {
                    match self.difficulty {
                        Difficulty::Special => {
                            // Stop regular game music if playing (safety check)
                            if self.game_music.playing() {
                                self.game_music.stop(ctx)?;
                            }
                            self.special_mode_music.play(ctx)?;
                        }
                        _ => {
                            // Stop special music if playing (safety check)
                            if self.special_mode_music.playing() {
                                self.special_mode_music.stop(ctx)?;
                            }
                            self.game_music.set_pitch(self.music_speed);
                            self.game_music.play(ctx)?;
                        }
                    }
                }

                self.last_update += ctx.time.delta().as_secs_f32();
                if self.last_update >= self.move_time {
                    self.last_update = 0.0;

                    // Check for and apply buffered input
                    if let Some(new_velocity) = self.next_velocity.take() {
                        self.velocity = new_velocity;
                    }

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
                        // Play eat sound
                        self.eat_sound.play(ctx)?;

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
                            // Play game over sound
                            self.game_over_sound.play(ctx)?;
                            // Stop game music on game over
                            self.game_music.set_pitch(1.0); // Reset pitch
                            self.game_music.stop(ctx)?;
                            // Stop special music if playing
                            if self.special_mode_music.playing() {
                                self.special_mode_music.stop(ctx)?;
                            }
                            break;
                        }
                    }

                    // Check for boundary collisions
                    if self.check_border_collisions() {
                        self.mode = GameMode::Menu;
                        // Play game over sound
                        self.game_over_sound.play(ctx)?;
                        // Stop game music on game over
                        self.game_music.set_pitch(1.0); // Reset pitch
                        self.game_music.stop(ctx)?;
                        // Stop special music if playing
                        if self.special_mode_music.playing() {
                            self.special_mode_music.stop(ctx)?;
                        }
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

                let mut volume_text = Text::new(format!(
                    "Volume: {:.0}% (+/- to change)",
                    self.music_volume * 100.0
                ));
                volume_text.set_scale(graphics::PxScale::from(40.0 * self.scale));
                canvas.draw(
                    &volume_text,
                    DrawParam::default()
                        .dest(mint::Point2 {
                            x: self.boundary_width * 0.5 - (300.0 * self.scale) + self.offset_x,
                            y: self.boundary_height * 0.75 + self.offset_y,
                        })
                        .color(Color::WHITE),
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
                );
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
                // Play sound on any relevant key press in the menu
                let mut play_sound = false;
                let mut volume_changed = false;

                if let Some(keycode) = key.keycode {
                    match keycode {
                        KeyCode::Return => {
                            // Start the game
                            self.mode = GameMode::Playing;
                            self.reset_game_state();
                            play_sound = true;
                        }
                        KeyCode::Escape => {
                            ctx.request_quit();
                            play_sound = true;
                        }
                        KeyCode::Key1 => {
                            // Set difficulty to Easy
                            if !matches!(self.difficulty, Difficulty::Easy) {
                                self.difficulty = Difficulty::Easy;
                                play_sound = true;
                            }
                        }
                        KeyCode::Key2 => {
                            // Set difficulty to Normal
                            if !matches!(self.difficulty, Difficulty::Normal) {
                                self.difficulty = Difficulty::Normal;
                                play_sound = true;
                            }
                        }
                        KeyCode::Key3 => {
                            // Set difficulty to Hard
                            if !matches!(self.difficulty, Difficulty::Hard) {
                                self.difficulty = Difficulty::Hard;
                                play_sound = true;
                            }
                        }
                        KeyCode::Key4 => {
                            // Set difficulty to Special
                            if !matches!(self.difficulty, Difficulty::Special) {
                                self.difficulty = Difficulty::Special;
                                play_sound = true;
                            }
                        }
                        KeyCode::Equals | KeyCode::Plus => {
                            // Increase volume
                            self.music_volume = (self.music_volume + 0.1).min(1.0);
                            volume_changed = true;
                        }
                        KeyCode::Minus => {
                            // Decrease volume
                            self.music_volume = (self.music_volume - 0.1).max(0.0);
                            volume_changed = true;
                        }
                        _ => {} // Ignore other keys
                    }
                }
                if play_sound {
                    // Stop the sound first to allow retriggering if pressed quickly
                    self.menu_change_sound.stop(ctx)?;
                    self.menu_change_sound.play(ctx)?;
                }
                if volume_changed {
                    self.menu_music.set_volume(self.music_volume);
                    self.game_music.set_volume(self.music_volume);
                    self.eat_sound.set_volume(self.music_volume);
                    self.game_over_sound.set_volume(self.music_volume);
                    self.special_mode_music.set_volume(self.music_volume);
                    self.menu_change_sound.stop(ctx)?;
                    self.menu_change_sound.play(ctx)?;
                }
            }
            GameMode::Playing => {
                let new_velocity = match key.keycode {
                    Some(KeyCode::Right) | Some(KeyCode::D) => na::Vector2::new(self.scaled_snake_size, 0.0),
                    Some(KeyCode::Left) | Some(KeyCode::A) => na::Vector2::new(-self.scaled_snake_size, 0.0),
                    Some(KeyCode::Up) | Some(KeyCode::W) => na::Vector2::new(0.0, -self.scaled_snake_size),
                    Some(KeyCode::Down) | Some(KeyCode::S) => na::Vector2::new(0.0, self.scaled_snake_size),
                    Some(KeyCode::Escape) => {
                        self.mode = GameMode::Menu;
                        // Reset pitch when escaping to menu
                        self.game_music.set_pitch(1.0);
                        // Stop special music if playing
                        if self.special_mode_music.playing() {
                            self.special_mode_music.stop(ctx)?;
                        }
                        return Ok(()); // Return early as we're switching mode
                    }
                    _ => return Ok(()), // Ignore other keys
                };

                // Prevent immediate reversal
                // Check against current velocity OR buffered velocity if it exists
                let current_check_velocity = self.next_velocity.unwrap_or(self.velocity);
                if new_velocity != -current_check_velocity {
                    self.next_velocity = Some(new_velocity);
                }
            }
        }
        Ok(())
    }
} 