use assets::Assets;

#[macroquad::test]
async fn assets_init() {
    Assets::init().await;
    let _assets = Assets::get();
}
