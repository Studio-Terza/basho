mod ports;

use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use guide::words::Spelling;
use tile_shapes::Corridor;

use crate::task::failure::Failure;
use crate::task::workspace::Workspace;
use basho_lints::Site;
use ports::Osmium;

struct CutName(usize);

impl CutName {
    fn path_in(self, cache: &Path) -> PathBuf {
        cache.join(self.to_string())
    }
}

impl fmt::Display for CutName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "cut{}.osm.pbf", self.0)
    }
}

pub struct TileSource {
    regions: Vec<PathBuf>,
    cache: PathBuf,
}

impl TileSource {
    const CACHE: &str = "target/tile-build";
    const FIRST_SOURCE_INDEX: usize = 2;
    const FILTERS: [&str; 16] = [
        "w/highway",
        "w/railway",
        "n/railway=station,halt",
        "w/waterway",
        "wr/natural",
        "w/route=ferry",
        "wr/landuse",
        "wr/leisure=park,nature_reserve,garden",
        "wr/building",
        "nwr/amenity",
        "nwr/shop",
        "nwr/tourism",
        "nwr/historic",
        "n/place",
        "w/aerialway",
        "r/waterway=riverbank",
    ];

    pub fn locate(workspace: &Workspace) -> Result<TileSource, Failure> {
        let regions: Vec<PathBuf> = env::args()
            .skip(Self::FIRST_SOURCE_INDEX)
            .map(PathBuf::from)
            .collect();
        if regions.is_empty() {
            Err(Failure::Usage(Spelling::new(
                "usage: cargo xtask tile-build <file.osm.pbf> ...".to_owned(),
            )))
        } else {
            Ok(TileSource {
                regions,
                cache: workspace.root().join(Self::CACHE),
            })
        }
    }

    pub fn features(&self, corridor: &Corridor) -> Result<PathBuf, Failure> {
        let failed = |error| Failure::Io {
            site: Site::from(self.cache.as_path()),
            error,
        };
        fs::create_dir_all(&self.cache).map_err(failed)?;
        let outline = self.cache.join("corridor.geojson");
        File::create(&outline)
            .and_then(|mut file| write!(file, "{}", corridor.outline()))
            .map_err(failed)?;
        let cuts: Vec<PathBuf> = (0..self.regions.len())
            .map(|index| CutName(index).path_in(&self.cache))
            .collect();
        self.regions
            .iter()
            .zip(&cuts)
            .try_for_each(|(region, cut)| Self::cut(region, &outline, cut))?;
        let merged = self.cache.join("merged.osm.pbf");
        Osmium::new(
            &[
                "merge".as_ref(),
                "--overwrite".as_ref(),
                "-o".as_ref(),
                merged.as_os_str(),
            ]
            .into_iter()
            .chain(cuts.iter().map(|cut| cut.as_os_str()))
            .collect::<Vec<&OsStr>>(),
        )
        .run()?;
        let kept = self.cache.join("kept.osm.pbf");
        Osmium::new(
            &["tags-filter".as_ref(), merged.as_os_str()]
                .into_iter()
                .chain(Self::FILTERS.iter().map(AsRef::as_ref))
                .chain(["--overwrite".as_ref(), "-o".as_ref(), kept.as_os_str()])
                .collect::<Vec<&OsStr>>(),
        )
        .run()?;
        let features = self.cache.join("features.geojsonseq");
        Osmium::new(&[
            "export".as_ref(),
            kept.as_os_str(),
            "-f".as_ref(),
            "geojsonseq".as_ref(),
            "-x".as_ref(),
            "print_record_separator=false".as_ref(),
            "--overwrite".as_ref(),
            "-o".as_ref(),
            features.as_os_str(),
        ])
        .run()?;
        Ok(features)
    }

    fn cut(region: &Path, outline: &Path, cut: &Path) -> Result<(), Failure> {
        Osmium::new(&[
            "extract".as_ref(),
            "-s".as_ref(),
            "complete_ways".as_ref(),
            "--overwrite".as_ref(),
            "-p".as_ref(),
            outline.as_os_str(),
            region.as_os_str(),
            "-o".as_ref(),
            cut.as_os_str(),
        ])
        .run()
    }
}
