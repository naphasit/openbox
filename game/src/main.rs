use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Openbox"),
        high_dpi: true,
        sample_count: 4,
        ..Conf::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        next_frame().await
    }
}
