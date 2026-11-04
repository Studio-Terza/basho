use map_class::Zoom;
use tiles::{TileArchive, TileId, WrittenArchive};

#[test]
fn a_written_tile_reads_back_and_a_missing_one_is_absent() {
    let kept = TileId::new(Zoom::MOST, 14_400, 6_300);
    let mut written = WrittenArchive::default();
    written.add(kept, b"road bytes").expect("the tile packs");
    written
        .add(TileId::new(Zoom::CELL, 7_200, 3_150), b"other")
        .expect("the tile packs");
    let archive = TileArchive::read(written.bytes()).expect("the archive opens");
    assert_eq!(archive.tiles_len(), 2);
    assert_eq!(
        archive.tile(kept).expect("reads").as_deref(),
        Some(b"road bytes".as_slice())
    );
    assert_eq!(
        archive.tile(TileId::new(Zoom::MOST, 1, 1)).expect("reads"),
        None
    );
}

#[test]
fn a_file_of_another_kind_does_not_open() {
    assert!(TileArchive::read(b"not tiles, and long enough to read".to_vec()).is_err());
}

#[test]
fn a_tile_name_parses_and_a_bad_one_is_refused() {
    let id: TileId = "14/14400/6300".parse().expect("a tile name");
    assert_eq!(id, TileId::new(Zoom::MOST, 14_400, 6_300));
    assert!("14/14400".parse::<TileId>().is_err());
    assert_eq!(id.parent(), Some(TileId::new(Zoom::CELL, 7_200, 3_150)));
}
