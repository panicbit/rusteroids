use ggez::GameResult;
use ggez::graphics;
use cgmath::prelude::*;
use rand::{Rand, Rng};

pub const TAU: f32 = 2.0 * ::std::f32::consts::PI;

pub type Point2 = ::cgmath::Point2<f32>;
pub type Vector2 = ::cgmath::Vector2<f32>;
pub type Basis2 = ::cgmath::Basis2<f32>;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Rand for Direction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        *rng.choose(&[Direction::Left, Direction::Right]).unwrap()
    }
}

pub trait ToGgez {
    type Output;

    fn to_ggez(&self) -> Self::Output;
}

impl ToGgez for Point2 {
    type Output = graphics::Point2;

    fn to_ggez(&self) -> Self::Output {
        graphics::Point2::new(self.x, self.y)
    }
}

#[derive(Debug)]
pub struct CommonProperties {
    pub position: Point2,
    pub velocity: Vector2,
    pub rotation: f32,
}

impl CommonProperties {
    pub fn new(position: Point2, velocity: Vector2, rotation: f32) -> Self {
        Self {
            position,
            velocity,
            rotation,
        }
    }
}

impl Default for CommonProperties {
    fn default() -> Self {
        Self::new(Point2::origin(), Vector2::zero(), 0.)
    }
}

pub fn for_each_wrap_offset<F: FnMut(Vector2) -> GameResult<()>>(
    width: f32,
    height: f32,
    mut f: F,
) -> GameResult<()> {
    // Center
    f(Vector2::zero())?;
    // Left
    f(Vector2::new(-width, 0.))?;
    // Right
    f(Vector2::new(width, 0.))?;
    // Top
    f(Vector2::new(0., -height))?;
    // Bottom
    f(Vector2::new(0., height))?;
    // Top Left
    f(Vector2::new(-width, -height))?;
    // Top Right
    f(Vector2::new(width, -height))?;
    // Bottom Left
    f(Vector2::new(-width, height))?;
    // Bottom Right
    f(Vector2::new(width, height))?;

    Ok(())
}
