use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use guide::Trip;

use crate::task::failure::Failure;
use crate::task::trip_text::TripText;
use crate::task::workspace::Workspace;
use basho_lints::Site;
use day_print::DaysHtml;

pub struct Print;

impl Print {
    const OUTPUT: &str = "target/print/basho-days.html";
}

impl Print {
    pub fn write_pages(workspace: &Workspace) -> Result<(), Failure> {
        let trip = Trip::load(&TripText::path_in(workspace))?;
        let itinerary = trip.itinerary();
        let path = workspace.root().join(Self::OUTPUT);
        let site = Site::from(Path::new(Self::OUTPUT));
        path.parent()
            .map_or(Ok(()), fs::create_dir_all)
            .and_then(|()| File::create(&path))
            .and_then(|mut file| write!(file, "{}", DaysHtml::new(itinerary)))
            .map_err(|error| Failure::Io { site, error })?;
        println!(
            "print: {} days written to {}",
            itinerary.days().len(),
            path.display()
        );
        Ok(())
    }
}
