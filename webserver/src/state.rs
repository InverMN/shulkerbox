use std::sync::atomic::AtomicUsize;
use rocket::{Build, Rocket};

pub trait InitStateExt {
    fn init_app_state(self) -> Self;
}

pub struct HitCount {
    count: AtomicUsize,
}

impl HitCount {
    pub fn get(&self) -> String {
        self.count.load(std::sync::atomic::Ordering::Relaxed).to_string()
    }

    pub fn increment(&self) {
        self.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    } 
}

impl InitStateExt for Rocket<Build> {
    fn init_app_state(self) -> Self {
        self
            .manage(HitCount { count: AtomicUsize::new(0) })
    }
}