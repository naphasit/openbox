use macroquad::prelude::*;

pub enum Texture {
    Dirt,
}

impl Into<Rect> for Texture {
    fn into(self) -> Rect {
        match self {
            Texture::Dirt => Rect::new(0.0, 0.0, 32.0, 32.0),
        }
    }
}
