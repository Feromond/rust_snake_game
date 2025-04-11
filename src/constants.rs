pub const REFERENCE_WIDTH: f32 = 1400.0;
pub const REFERENCE_HEIGHT: f32 = 1050.0;
pub const MIN_WINDOW_WIDTH: f32 = 800.0;
pub const MIN_WINDOW_HEIGHT: f32 = 600.0;
pub const REFERENCE_SNAKE_SIZE: f32 = 50.0;

// Controls snake speed.
pub const EASY_MOVE_TIME: f32 = 0.12;
pub const NORMAL_MOVE_TIME: f32 = 0.08;
pub const HARD_MOVE_TIME: f32 = 0.065;

// Special mode constants
pub const SPECIAL_START_MOVE_TIME: f32 = 0.15; // Initial slower speed for Special
pub const SPEED_UP_FACTOR: f32 = 0.95; // Multiplier for the speed increase
pub const MIN_MOVE_TIME: f32 = 0.03; // Minimum move time to avoid it being too fast

// Helps remove floating point errors
pub const EPSILON: f32 = 0.01; 

// Audio Constants
pub const INITIAL_MUSIC_VOLUME: f32 = 0.8;