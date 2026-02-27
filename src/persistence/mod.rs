mod load;
mod save;

use std::path::PathBuf;

pub use load::load_tracker;
pub use save::save_tracker;

fn data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join("med-tracker"))
}
