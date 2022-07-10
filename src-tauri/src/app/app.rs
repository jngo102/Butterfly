use crate::app::settings::Settings;
use threadpool::ThreadPool;

pub struct App {
    pub current_download_progress: u8,
    pub settings: Settings,
    pub pool: ThreadPool,
}

impl Default for App {
    fn default() -> Self {
        App {
            current_download_progress: 0,
            settings: Settings::default(),
            pool: ThreadPool::new(num_cpus::get()),
        }
    }
}