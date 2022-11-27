//! Contains functions for managing static files such as image uploads.

use rand::distributions::{Alphanumeric, DistString};
use std::path::PathBuf;

/// returns a filename and full path
/// ext should include the period
pub fn generate_image_path(ext: &str) -> (PathBuf, String) {
    let filename = generate_filename(ext);
    (PathBuf::from(format!("static/image/{filename}")), filename)
}

/// returns a filename and full path
/// ext should include the period
pub fn generate_thumbnail_path(ext: &str) -> (PathBuf, String) {
    let filename = generate_filename(ext);
    (
        PathBuf::from(format!("static/thumbnail/{filename}")),
        filename,
    )
}

/// returns a filename and full path
/// ext should include the period
pub fn generate_avatar_path(ext: &str) -> (PathBuf, String) {
    let filename = generate_filename(ext);
    (PathBuf::from(format!("static/avatar/{filename}")), filename)
}

fn generate_filename(ext: &str) -> String {
    let mut name = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    name.push_str(ext);
    name
}
