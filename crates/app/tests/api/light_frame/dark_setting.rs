use basho::app::session::DarkAfterSunset;
use storage_fake::MemoryStorage;

#[test]
fn the_dark_setting_starts_off_and_comes_back_as_it_was_kept() {
    let home = Some(MemoryStorage::home());
    assert_eq!(
        DarkAfterSunset::restored_in(home.as_ref()),
        DarkAfterSunset::Off
    );
    DarkAfterSunset::On.kept_in(home.as_ref()).unwrap();
    assert_eq!(
        DarkAfterSunset::restored_in(home.as_ref()),
        DarkAfterSunset::On
    );
    DarkAfterSunset::On
        .switched()
        .kept_in(home.as_ref())
        .unwrap();
    assert_eq!(
        DarkAfterSunset::restored_in(home.as_ref()),
        DarkAfterSunset::Off
    );
}
