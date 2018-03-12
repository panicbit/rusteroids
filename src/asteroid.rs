use util::*;
use cgmath::prelude::*;
use cgmath::Rad;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::DrawMode::Fill;
use super::State;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

pub struct Asteroid {
    common: CommonProperties,
    rotation_direction: Direction,
}

impl Asteroid {
    pub fn new(ctx: &Context) -> Self {
        let ref mut rng = ::rand::thread_rng();

        let x_range = Range::new(0., ctx.conf.window_mode.width as f32);
        let y_range = Range::new(0., ctx.conf.window_mode.height as f32);
        let rot_range = Range::new(0., TAU);

        let rotation = rot_range.ind_sample(rng);
        let basis = Basis2::from_angle(Rad(rotation));
        let velocity = Vector2::new(0.4, 0.4);
        let velocity = basis.rotate_vector(velocity);

        let common = CommonProperties::new(
            Point2::new(x_range.ind_sample(rng), y_range.ind_sample(rng)),
            velocity,
            rotation,
        );

        Self {
            common,
            rotation_direction: rng.gen::<Direction>(),
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        match self.rotation_direction {
            Direction::Left => self.common.rotation -= 0.02,
            Direction::Right => self.common.rotation += 0.02,
        }

        self.common.position += self.common.velocity;

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, state: &State) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            &state.assets.asteroid,
            DrawParam {
                dest: self.common.position.to_ggez(),
                scale: Point2::new(0.1, 0.1).to_ggez(),
                offset: Point2::new(0.5, 0.5).to_ggez(),
                rotation: self.common.rotation,
                ..Default::default()
            },
        )?;

        Ok(())
    }
}
