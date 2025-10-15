use assets::Texture;
use macroquad::prelude::*;

#[test]
fn texture_get() {
    let dirt: Rect = Texture::Dirt.into();

    assert_eq!(dirt, Rect::new(0.0, 0.0, 32.0, 32.0))
}
