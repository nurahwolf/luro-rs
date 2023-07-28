use crate::models::Hecks;

use super::toml::LuroTOML;

impl LuroTOML for Hecks {}

impl Hecks {
    /// Reload sfw heck IDs
    pub fn reload_sfw_heck_ids(&mut self) {
        let mut heck_ids = vec![];

        for num in 0..self.sfw_hecks.len() {
            heck_ids.push(num)
        }

        self.sfw_heck_ids = heck_ids;
    }

    /// Reload nsfw heck IDs
    pub fn reload_nsfw_heck_ids(&mut self) {
        let mut heck_ids = vec![];

        for num in 0..self.nsfw_hecks.len() {
            heck_ids.push(num)
        }

        self.nsfw_heck_ids = heck_ids;
    }
}
