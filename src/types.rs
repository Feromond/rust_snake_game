use nalgebra as na;

#[derive(PartialEq, Clone, Copy)]
pub enum GameMode {
    Menu,
    Playing,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Special,
}

#[derive(Clone, Copy)]
pub struct Food {
    pub pos: na::Point2<f32>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeSegment {
    pub pos: na::Point2<f32>,
} 