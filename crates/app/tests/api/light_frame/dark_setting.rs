use basho::app::session::DarkAfterSunset;

#[test]
fn the_dark_setting_starts_off_and_comes_back_as_it_was_kept() {
    let home = tempfile::tempdir().unwrap();
    assert_eq!(
        DarkAfterSunset::restored_under(home.path()),
        DarkAfterSunset::Off
    );
    DarkAfterSunset::On.kept_under(home.path()).unwrap();
    assert_eq!(
        DarkAfterSunset::restored_under(home.path()),
        DarkAfterSunset::On
    );
    DarkAfterSunset::On
        .switched()
        .kept_under(home.path())
        .unwrap();
    assert_eq!(
        DarkAfterSunset::restored_under(home.path()),
        DarkAfterSunset::Off
    );
}
