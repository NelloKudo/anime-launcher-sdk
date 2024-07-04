use std::path::{Path, PathBuf};

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use anime_game_core::zzz::consts::GameEdition;

use crate::zzz::consts::launcher_dir;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paths {
    pub global: PathBuf,
    pub china: PathBuf
}

impl Paths {
    #[inline]
    /// Get game path for given edition
    pub fn for_edition(&self, edition: impl Into<GameEdition>) -> &Path {
        match edition.into() {
            GameEdition::Global => self.global.as_path(),
            GameEdition::China => self.china.as_path()
        }
    }
}

impl Default for Paths {
    fn default() -> Self {
        let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

        Self {
            global: launcher_dir.join(concat!("Zen", "less Z", "one Zero")),
            china: launcher_dir.join(concat!("Zen", "less Z", "one Zero"))
        }
    }
}

impl From<&JsonValue> for Paths {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            global: value.get("global")
                .and_then(JsonValue::as_str)
                .map(PathBuf::from)
                .unwrap_or(default.global),

            china: value.get("china")
                .and_then(JsonValue::as_str)
                .map(PathBuf::from)
                .unwrap_or(default.china),
        }
    }
}
