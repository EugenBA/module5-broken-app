use std::sync::{Arc};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

pub struct Counter {
    value: Arc<AtomicU64>
}

/// Используем unsafe и arc
impl Counter {
    pub fn new() -> Self {
        Self { value: Arc::from(AtomicU64::new(0)) }
    }
    pub fn race_increment(&self, iterations: usize, threads: usize) -> u64{
        self.value.store(0, Ordering::Relaxed);
        let mut handles = Vec::new();
        for _ in 0..threads {
            let counter = Arc::clone(&self.value);
            handles.push(thread::spawn(move || {
                for _ in 0..iterations {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }
        for h in handles {
            let _ = h.join();
        }
        self.value.load(Ordering::Relaxed)
    }

    /// синхронизация через mutex
    pub fn read_after_sleep(&self) -> u64 {
        thread::sleep(Duration::from_millis(10));
       self.value.load(Ordering::Relaxed)
    }

    /// Сброс счётчика (также небезопасен, без синхронизации).
    pub fn reset_counter(&self) {
        self.value.store(0, Ordering::Relaxed);
    }
}
