extern crate actix;
extern crate cgmath;
extern crate futures;
extern crate ggez;
extern crate rand;
extern crate tokio;

use std::collections::HashSet;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Text};
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::timer;

use util::*;
use world::World;
use assets::Assets;

mod util;
mod assets;
mod world;
mod ship;
mod asteroid;

fn main() {
    let ref mut ctx = ContextBuilder::new("rusteroids", "panicbit")
        .build()
        .unwrap();

    let ref mut state = State::new(ctx).unwrap();

    event::run(ctx, state).unwrap();
}

pub struct State {
    assets: Assets,
    pressed_keys: HashSet<Keycode>,
    world: World,
    debug_mode: bool,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            assets: Assets::load(ctx)?,
            pressed_keys: HashSet::new(),
            world: World::new(ctx),
            debug_mode: true,
        })
    }

    fn turning_left(&self) -> bool {
        self.pressed_keys.contains(&Keycode::Left) || self.pressed_keys.contains(&Keycode::I)
    }

    fn turning_right(&self) -> bool {
        self.pressed_keys.contains(&Keycode::Right) || self.pressed_keys.contains(&Keycode::E)
    }

    fn moving_forwards(&self) -> bool {
        self.pressed_keys.contains(&Keycode::Up) || self.pressed_keys.contains(&Keycode::L)
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.world.update(ctx)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.world.draw(ctx, self)?;

        if self.debug_mode {
            // FPSfn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
            let fps = timer::get_fps(ctx);
            let fps = format!("{:.0}", fps);
            let fps = Text::new(ctx, &fps, &self.assets.font)?;
            graphics::draw(ctx, &fps, Point2::new(5., 0.).to_ggez(), 0.)?;

            // Draw axis with origin at (100,100)
            let points = &[
                Point2::new(100., 0.).to_ggez(),
                Point2::new(100., 200.).to_ggez(),
            ];
            graphics::line(ctx, points, 2.0)?;

            let points = &[
                Point2::new(0., 100.).to_ggez(),
                Point2::new(200., 100.).to_ggez(),
            ];
            graphics::line(ctx, points, 2.0)?;
        }

        graphics::present(ctx);

        timer::yield_now();

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }

        match keycode {
            Keycode::F2 => self.debug_mode = !self.debug_mode,
            Keycode::Escape => ctx.quit().unwrap(),
            Keycode::Left | Keycode::I => self.world.set_ship_turning(Direction::Left),
            Keycode::Right | Keycode::E => self.world.set_ship_turning(Direction::Right),
            Keycode::Up | Keycode::L => self.world.set_ship_thruster_on(true),
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }

        let dir = self.world.get_ship_turning();

        match keycode {
            Keycode::Left | Keycode::I if dir == Some(Direction::Left) => {
                self.world.set_ship_turning(None)
            }
            Keycode::Right | Keycode::E if dir == Some(Direction::Right) => {
                self.world.set_ship_turning(None)
            }
            Keycode::Up | Keycode::L => self.world.set_ship_thruster_on(false),
            _ => {}
        }
    }
}
