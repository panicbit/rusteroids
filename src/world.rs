use ggez::{Context, GameResult};
use ship::Ship;
use asteroid::Asteroid;
use super::State;
use util::*;

pub struct World {
    ships: Vec<Ship>,
    asteroids: Vec<Asteroid>,
}

impl World {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            ships: vec![Ship::new()],
            asteroids: (0..20).map(|_| Asteroid::new(ctx)).collect(),
        }
    }

    pub fn set_ship_turning<T: Into<Option<Direction>>>(&mut self, dir: T) {
        self.ships[0].set_turning(dir);
    }

    pub fn set_ship_thruster_on(&mut self, state: bool) {
        self.ships[0].set_thruster_on(state);
    }

    pub fn get_ship_turning(&self) -> Option<Direction> {
        self.ships[0].get_turning()
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for ship in &mut self.ships {
            ship.update(ctx)?;
        }

        for asteroid in &mut self.asteroids {
            asteroid.update(ctx)?;
        }

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, state: &State) -> GameResult<()> {
        for asteroid in &self.asteroids {
            asteroid.draw(ctx, state)?;
        }

        for ship in &self.ships {
            ship.draw(ctx, state)?;
        }

        Ok(())
    }
}
