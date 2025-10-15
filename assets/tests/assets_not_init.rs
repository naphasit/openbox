use assets::Assets;

#[test]
#[should_panic]
fn assets_not_init() {
    let _assets = Assets::get();
}
