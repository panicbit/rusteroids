use ggez::{Context, GameResult};
use ggez::graphics::{Font, Image};

pub struct Assets {
    pub ship: Image,
    pub asteroid: Image,
    pub font: Font,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            ship: Image::new(ctx, "/ship.png")?,
            asteroid: Image::new(ctx, "/gopher.png")?,
            font: Font::new(ctx, "/Computer Speak v0.3.ttf", 12)?,
        })
    }
}
