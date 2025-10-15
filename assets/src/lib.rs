use std::sync::OnceLock;

use macroquad::prelude::*;

pub struct Assets {
    pub texture: Texture2D,
}

pub static ASSETS: OnceLock<Assets> = OnceLock::new();

impl Assets {
    pub async fn init() {
        info!("Init assets");
        let assets = Assets {
            texture: Texture2D::empty(),
        };

        ASSETS
            .set(assets)
            .unwrap_or_else(|_| error!("Can't init assets"));
    }

    pub fn get() -> &'static Assets {
        ASSETS.get().unwrap_or_else(|| {
            trace!("Get assets");
            panic!("Assets not initialized yet");
        })
    }
}

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
