mod tile_source;

use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use guide::Trip;
use osm_items::OsmLine;
use tile_draft::Pyramid;
use tile_shapes::Corridor;

use crate::task::failure::Failure;
use crate::task::trip_text::TripText;
use crate::task::workspace::Workspace;
use basho_lints::Site;
use tile_source::TileSource;

pub struct TileBuild;

impl TileBuild {
    const ARCHIVE: &str = "crates/app/assets/basho.tiles";

    pub fn write_archive(workspace: &Workspace) -> Result<(), Failure> {
        let path = TripText::path_in(workspace);
        let text = fs::read_to_string(&path).map_err(|error| Failure::Io {
            site: Site::from(path.as_path()),
            error,
        })?;
        let corridor = Corridor::of_trip(&text.parse::<Trip>()?);
        let features = TileSource::locate(workspace)?.features(&corridor)?;
        let mut pyramid = Pyramid::new(corridor);
        Self::taken_into(&features, &mut pyramid)?;
        let archive: PathBuf = workspace.root().join(Self::ARCHIVE);
        let failed = |error| Failure::Io {
            site: Site::from(archive.as_path()),
            error,
        };
        let written = pyramid.archive().map_err(failed)?;
        fs::write(&archive, written.bytes()).map_err(failed)
    }

    fn taken_into(features: &Path, pyramid: &mut Pyramid) -> Result<(), Failure> {
        let failed = |error| Failure::Io {
            site: Site::from(features),
            error,
        };
        BufReader::new(File::open(features).map_err(failed)?)
            .lines()
            .try_for_each(|line| {
                let parsed: OsmLine = line.map_err(failed)?.parse().map_err(Failure::MapData)?;
                parsed
                    .tile_item()
                    .iter()
                    .for_each(|item| pyramid.take(item));
                Ok(())
            })
    }
}
