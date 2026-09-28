//! Centralized derivation of Tinty's on-disk data-directory layout.
//!
//! Every operation locates installed template repositories and the built-in
//! schemes repo under `<data_dir>/repos/`. These helpers are the single source
//! of truth for that layout, so call sites don't each re-spell the
//! `repos/<name>` join (previously duplicated across nearly every operation).
//!
//! These take an already-resolved `data_path` (tilde expansion for the data
//! directory happens once at startup in `main`); they do no `~` expansion of
//! their own.

use crate::config::ORG_NAME;
use crate::constants::{REPO_DIR, REPO_NAME, SCHEMES_REPO_NAME};
use std::path::{Path, PathBuf};

/// The directory holding every installed repository: `<data_dir>/repos`.
pub fn repos_dir(data_path: &Path) -> PathBuf {
    data_path.join(REPO_DIR)
}

/// The local path of an installed `[[items]]` template repository, by item
/// name: `<data_dir>/repos/<name>`.
pub fn item_repo_path(data_path: &Path, item_name: &str) -> PathBuf {
    repos_dir(data_path).join(item_name)
}

/// The local path of the built-in schemes repository: `<data_dir>/repos/schemes`.
pub fn schemes_repo_path(data_path: &Path) -> PathBuf {
    repos_dir(data_path).join(SCHEMES_REPO_NAME)
}

/// The tinty config directory: `<config_home>/tinted-theming/tinty`.
pub fn config_dir() -> PathBuf {
    let base = dirs::config_dir().or_else(dirs::data_dir)
        .expect("unable to resolve config dir");
    base.join(format!("{ORG_NAME}/{REPO_NAME}"))
}

/// The tinty data directory: `<data_home>/tinted-theming/tinty`.
pub fn data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("unable to resolve data dir")
        .join(format!("{ORG_NAME}/{REPO_NAME}"))
}

