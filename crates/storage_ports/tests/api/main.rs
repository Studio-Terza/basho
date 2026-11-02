use std::fs;
use std::path::PathBuf;

use laws::proptest::prelude::{any, prop_assert_eq};
use storage_ports::HomeFolder;

const KEPT: [&str; 3] = ["Library", "basho", "kept.txt"];
const ROOTS: &str = "/[ab]";

#[test]
fn a_replaced_file_on_disk_is_whole_and_leaves_no_part_file_behind() {
    let root = tempfile::tempdir().unwrap();
    let home = HomeFolder::on_disk(root.path().to_path_buf());
    let path = home.joined(KEPT);
    home.replace(&path, &"old").unwrap();
    home.replace(&path, &"new\nlines").unwrap();
    assert_eq!(home.read(&path).unwrap(), "new\nlines");
    assert_eq!(
        home.names_in(path.parent().unwrap()).unwrap(),
        vec![KEPT[2]]
    );
    home.forget(&path).unwrap();
    assert!(!path.exists());
}

#[test]
fn a_write_that_fails_part_way_leaves_the_old_file_whole() {
    let root = tempfile::tempdir().unwrap();
    let home = HomeFolder::on_disk(root.path().to_path_buf());
    let path = home.joined(KEPT);
    home.replace(&path, &"old").unwrap();
    fs::create_dir(path.with_extension("part")).unwrap();
    assert!(home.replace(&path, &"new").is_err());
    assert_eq!(fs::read_to_string(&path).unwrap(), "old");
}

#[test]
fn the_user_home_is_the_home_folder_on_disk() {
    assert_eq!(
        HomeFolder::of_user().map(|user| user.joined(KEPT)),
        std::env::var_os("HOME").map(|user| PathBuf::from(user).join(KEPT.join("/")))
    );
}

laws::law!(
    homes_are_equal_only_with_one_root_and_one_storage,
    (ROOTS, ROOTS, any::<bool>()),
    case,
    {
        let (left, right, shared) = case;
        let first = HomeFolder::on_disk(PathBuf::from(&left));
        let second = if shared {
            first.clone()
        } else {
            HomeFolder::on_disk(PathBuf::from(&right))
        };
        prop_assert_eq!(first == second, shared);
    }
);
