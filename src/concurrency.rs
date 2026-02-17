use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Counter {
    value: Arc<Mutex<u64,>>
}

/// Используем unsafe и arc
impl Counter {
    pub fn new() -> Self {
        Self { value: Arc::new(Mutex::new(0)) }
    }
    pub fn race_increment(&self, iterations: usize, threads: usize) -> Option<u64>{
        if let Ok(mut lock) = self.value.lock() {
            *lock = 0;
        }
        let mut handles = Vec::new();
        for _ in 0..threads {
            let counter = self.value.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..iterations {
                    if let Ok(mut lock) = counter.lock() {
                        *lock += 1;
                    }
                }
            }));
        }
        for h in handles {
            let _ = h.join();
        }
        if let Ok(lock) = self.value.lock() {
            Some(*lock)
        }
        else {
            None
        }
    }

    /// синхронизация через mutex
    pub fn read_after_sleep(&self) -> Option<u64> {
        thread::sleep(Duration::from_millis(10));
        if let Ok(lock) = self.value.lock() {
            Some(*lock)
        }
        else {
            None
        }
    }

    /// Сброс счётчика (также небезопасен, без синхронизации).
    pub fn reset_counter(&self) {
        if let Ok(mut lock) = self.value.lock() {
            *lock = 0;
        }
    }
}
