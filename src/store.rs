use super::stats::PathStats;
use super::current_time_secs;
use super::SortMethod;
use std::default::Default;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::time::SystemTime;

pub struct UsageStore {
    pub reference_time: f64,
    pub half_life: f32,
    pub paths: Vec<PathStats>,
}

impl Default for UsageStore {
    fn default() -> UsageStore {
        UsageStore {
            reference_time: current_time_secs(),
            half_life: 60.0 * 60.0 * 24.0 * 3.0, // three day half life
            paths: Vec::new(),
        }
    }
}

impl UsageStore {
    pub fn purge(&mut self) {
        self.paths.retain(|dir| Path::new(&dir.path).exists());
    }

    pub fn sorted(&self, sort_method: &SortMethod) -> Vec<PathStats> {
        let mut new_vec = self.paths.clone();

        new_vec.sort_by(|dir1, dir2| dir1.cmp_score(dir2, sort_method).reverse());

        new_vec
    }

    pub fn truncate(&mut self, keep_num: usize, sort_method: &SortMethod) {
        let mut sorted_vec = self.sorted(sort_method);
        sorted_vec.truncate(keep_num);
        self.paths = sorted_vec;
    }

    pub fn reset_time(&mut self, time: SystemTime) {
        unimplemented!();
    }

    pub fn add(&mut self, path: &str) {
        let path_stats = self.get(&path);
        path_stats.update_score(1.0);
        path_stats.update_num_accesses(1);
        path_stats.update_last_access();
    }

    pub fn adjust(&mut self, path: &str, weight: f32) {
        let path_stats = self.get(&path);
        path_stats.update_score(weight);
        path_stats.update_num_accesses(weight as i32);
    }

    fn get(&mut self, path: &str) -> &mut PathStats {
        match self.paths.binary_search_by_key(&path, |dir_stats| &dir_stats.path) {
            Ok(idx) => &mut self.paths[idx],
            Err(idx) => {
                self.paths.insert(
                    idx,
                    PathStats::new(path.to_string(), self.reference_time, self.half_life),
                );
                &mut self.paths[idx]
            }
        }
    }

    pub fn print_sorted(&self, method: &SortMethod, show_stats: bool, limit: Option<usize>) {

        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut writer = BufWriter::new(handle);

        let sorted = self.sorted(method);
        let take_num = limit.unwrap_or(sorted.len());

        for dir in sorted.iter().take(take_num) {
          writer.write_all(dir.to_string(method, show_stats).as_bytes())
            .expect("Unable to write to stdout");
        }
    }
}

