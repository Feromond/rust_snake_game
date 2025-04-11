use crate::constants::*;
use crate::types::*;
use ggez::{
    audio::{self, SoundSource, Source},
    graphics::{Canvas, Color, DrawMode, MeshBuilder, Rect},
    Context,
    GameResult,
};
use nalgebra as na;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct GameState {
    pub snake_body: Vec<SnakeSegment>,
    pub food: Food,
    pub velocity: na::Vector2<f32>,
    pub last_update: f32,
    pub score: i32,
    pub high_score: i32,
    pub mode: GameMode,
    pub window_width: f32,
    pub window_height: f32,
    pub scale: f32,
    pub boundary_width: f32,
    pub boundary_height: f32,
    pub scaled_snake_size: f32,
    pub offset_x: f32, // Offset to center the game in case of extra window space
    pub offset_y: f32,
    pub difficulty: Difficulty,
    pub move_time: f32, // Store the current move time based on difficulty
    pub next_velocity: Option<na::Vector2<f32>>, // Buffer for the next move input
    // Audio fields
    pub menu_music: Source,
    pub game_music: Source,
    pub eat_sound: Source,
    pub game_over_sound: Source,
    pub menu_change_sound: Source,
    pub music_volume: f32,
    pub music_speed: f32,
    pub special_mode_music: Source,
}

impl GameState {
    pub fn new(ctx: &mut Context, is_bundle: bool) -> GameResult<GameState> {
        let (window_width, window_height) = GameState::get_window_size(ctx);
        let (boundary_width, boundary_height) =
            GameState::calculate_locked_boundary(window_width, window_height);
        let scale = boundary_width / REFERENCE_WIDTH;
        let scaled_snake_size = REFERENCE_SNAKE_SIZE * scale;
        let offset_x = (window_width - boundary_width) / 2.0;
        let offset_y = (window_height - boundary_height) / 2.0;

        // Determine path prefix based on context
        let resource_prefix = if is_bundle { "/resources" } else { "" };

        // Load audio files using the determined prefix
        let mut menu_music =
            audio::Source::new(ctx, &format!("{}{}", resource_prefix, "/menu_music.mp3"))?;
        let mut game_music =
            audio::Source::new(ctx, &format!("{}{}", resource_prefix, "/game_music.wav"))?;
        let mut eat_sound =
            audio::Source::new(ctx, &format!("{}{}", resource_prefix, "/eat.ogg"))?;
        let mut game_over_sound =
            audio::Source::new(ctx, &format!("{}{}", resource_prefix, "/game_over.wav"))?;
        let menu_change_sound = audio::Source::new(
            ctx,
            &format!("{}{}", resource_prefix, "/menu_option_change.wav"),
        )?;
        let mut special_mode_music = audio::Source::new(
            ctx,
            &format!("{}{}", resource_prefix, "/special_mode_music.mp3"),
        )?;

        // Set music to loop
        menu_music.set_repeat(true);
        game_music.set_repeat(true);
        special_mode_music.set_repeat(true);
        menu_music.set_volume(INITIAL_MUSIC_VOLUME);
        game_music.set_volume(INITIAL_MUSIC_VOLUME);
        eat_sound.set_volume(INITIAL_MUSIC_VOLUME);
        game_over_sound.set_volume(INITIAL_MUSIC_VOLUME);
        special_mode_music.set_volume(INITIAL_MUSIC_VOLUME);
        // Play menu music initially
        menu_music.play(ctx)?;

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
            next_velocity: None,
            // Initialize audio fields
            menu_music,
            game_music,
            eat_sound,
            game_over_sound,
            menu_change_sound,
            music_volume: INITIAL_MUSIC_VOLUME,
            music_speed: 1.0,
            special_mode_music,
        };
        Ok(s)
    }

    pub fn calculate_locked_boundary(window_width: f32, window_height: f32) -> (f32, f32) {
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

    pub fn get_random_food_position(
        boundary_width: f32,
        boundary_height: f32,
        scale: f32,
    ) -> na::Point2<f32> {
        let mut rng: ThreadRng = rand::rng();
        na::Point2::new(
            (rng.random_range(0..(boundary_width / (REFERENCE_SNAKE_SIZE * scale)) as u32) as f32)
                * (REFERENCE_SNAKE_SIZE * scale),
            (rng.random_range(0..(boundary_height / (REFERENCE_SNAKE_SIZE * scale)) as u32) as f32)
                * (REFERENCE_SNAKE_SIZE * scale),
        )
    }

    pub fn reset_game_state(&mut self) {
        self.snake_body = vec![SnakeSegment {
            pos: na::Point2::new(
                ((self.boundary_width / 4.0) / self.scaled_snake_size).floor()
                    * self.scaled_snake_size,
                ((self.boundary_height / 4.0) / self.scaled_snake_size).floor()
                    * self.scaled_snake_size,
            ),
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
        // Set initial music speed based on difficulty
        self.music_speed = match self.difficulty {
            Difficulty::Easy => 0.8,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.2,
            Difficulty::Special => 1.0,
        };
    }

    pub fn get_window_size(ctx: &mut Context) -> (f32, f32) {
        let canvas = Canvas::from_frame(ctx, Some(Color::BLACK));
        let rect = canvas.screen_coordinates().unwrap();
        (rect.w, rect.h)
    }

    pub fn handle_resize(&mut self, ctx: &mut Context) {
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

    pub fn scaled_rect(&self, pos: na::Point2<f32>) -> Rect {
        Rect::new(
            self.offset_x + pos.x,
            self.offset_y + pos.y,
            self.scaled_snake_size,
            self.scaled_snake_size,
        )
    }

    pub fn draw_border(&self, mesh_builder: &mut MeshBuilder) {
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

    pub fn check_border_collisions(&self) -> bool {
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