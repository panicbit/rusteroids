use util::*;
use cgmath::prelude::*;
use cgmath::Rad;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::DrawMode::Fill;
use super::State;

const ROTATION_SPEED: f32 = 0.06;
const ACCELERATION: f32 = 0.1;

pub struct Ship {
    common: CommonProperties,
    turning: Option<Direction>,
    thrusters_on: bool,
    name: String,
}

impl Ship {
    pub fn new() -> Self {
        let common = CommonProperties::new(Point2::new(100., 100.), Vector2::zero(), TAU / 2.);

        Self {
            common,
            turning: None,
            thrusters_on: false,
            name: "player".into(),
        }
    }

    pub fn get_turning(&self) -> Option<Direction> {
        self.turning
    }

    pub fn set_turning<T: Into<Option<Direction>>>(&mut self, dir: T) {
        self.turning = dir.into();
    }

    pub fn set_thruster_on(&mut self, state: bool) {
        self.thrusters_on = state;
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Rotate ship
        if self.turning == Some(Direction::Left) {
            self.common.rotation -= ROTATION_SPEED;
        } else if self.turning == Some(Direction::Right) {
            self.common.rotation += ROTATION_SPEED;
        }

        // Ensure rotation is between -TAU and TAU
        self.common.rotation %= TAU;

        // Rotate acceleration vector
        let accel = Vector2::new(0., -ACCELERATION);
        let rot = Basis2::from_angle(Rad(self.common.rotation));
        let accel = rot.rotate_vector(accel);

        // Apply acceleration to velocity
        if self.thrusters_on {
            self.common.velocity += accel;
        }

        // Apply velocity to position
        self.common.position += self.common.velocity;

        // Wrap around at window borders
        let width = ctx.conf.window_mode.width as f32;
        let height = ctx.conf.window_mode.height as f32;
        self.common.position.x = (width + self.common.position.x) % width;
        self.common.position.y = (height + self.common.position.y) % height;

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, state: &State) -> GameResult<()> {
        let width = ctx.conf.window_mode.width as f32;
        let height = ctx.conf.window_mode.height as f32;

        let basis = Basis2::from_angle(Rad(self.common.rotation));

        let params = DrawParam {
            scale: Point2::new(0.2, 0.2).to_ggez(),
            offset: Point2::new(0.5, 0.5).to_ggez(),
            rotation: self.common.rotation,
            ..Default::default()
        };

        let mut ship = SpriteBatch::new(state.assets.ship.clone());
        for_each_wrap_offset(width, height, |offset| {
            ship.add(DrawParam {
                dest: Point2::from_vec(offset).to_ggez(),
                ..params
            });

            Ok(())
        })?;

        graphics::draw(ctx, &ship, self.common.position.to_ggez(), 0.)?;

        let name = Text::new(ctx, &self.name, &state.assets.font)?;
        let name_offset = basis.rotate_vector(Vector2::new(0., -20.));

        for_each_wrap_offset(width, height, |offset| {
            graphics::draw_ex(
                ctx,
                &name,
                DrawParam {
                    dest: (self.common.position + offset + name_offset).to_ggez(),
                    color: Some(Color::new(0.9, 0.9, 0.9, 0.9)),
                    offset: Point2::new(0.5, 0.5).to_ggez(),
                    rotation: self.common.rotation,
                    ..Default::default()
                },
            )
        })?;

        if state.debug_mode {
            // Mark position
            graphics::circle(ctx, Fill, self.common.position.to_ggez(), 5., 0.5)?;
        }

        Ok(())
    }
}
