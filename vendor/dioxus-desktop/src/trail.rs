//! A short trail of lifecycle events, written to `Documents/trail.log` so it can
//! be read in the Files app after something went wrong on a device.
//!
//! Everything here is best effort: a failure to write is ignored, because the
//! trail must never be the thing that breaks the app. Writes from different
//! threads are serialised so lines never interleave.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Past this size the trail is moved aside to `trail.old.log` and started again.
const ROLL_BYTES: u64 = 256 * 1024;

static WRITER: Mutex<()> = Mutex::new(());

fn trail_path() -> Option<PathBuf> {
    if !cfg!(target_os = "ios") {
        return None;
    }
    let home = std::env::var_os("HOME")?;
    let documents = PathBuf::from(home).join("Documents");
    fs::create_dir_all(&documents).ok()?;
    Some(documents.join("trail.log"))
}

/// Append one line to the trail, stamped with seconds since the Unix epoch.
pub(crate) fn note(line: impl AsRef<str>) {
    tracing::info!("trail: {}", line.as_ref());
    let Some(path) = trail_path() else {
        return;
    };
    let Ok(_writing) = WRITER.lock() else {
        return;
    };
    if fs::metadata(&path).is_ok_and(|meta| meta.len() > ROLL_BYTES) {
        _ = fs::rename(&path, path.with_file_name("trail.old.log"));
    }
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs_f64())
        .unwrap_or_default();
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
        _ = file.write_all(format!("{seconds:.1} {}\n", line.as_ref()).as_bytes());
    }
}
