use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct CountingTimer {
    count: Arc<Mutex<u8>>,
    abort: Arc<Mutex<bool>>,
}

impl CountingTimer {
    pub fn new(count: u8) -> CountingTimer {
        CountingTimer {
            count: Arc::new(Mutex::new(count)),
            abort: Arc::new(Mutex::new(false)),
        }
    }
}

impl CountingTimer {
    pub fn abort<F>(&mut self, f: F)
    where
        F: Fn(u8),
    {
        *self.abort.lock().unwrap() = true;
        let c = *self.count.lock().unwrap();
        f(c);
    }

    pub fn start<B, A>(&mut self, before_each: B, after_each: A)
    where
        B: Fn(u8) + Send + 'static,
        A: Fn(u8) + Send + 'static,
    {
        thread::spawn({
            let count = Arc::clone(&self.count);
            let abort = Arc::clone(&self.abort);
            move || {
                let start = *count.lock().unwrap();
                for _ in 0..start {
                    thread::sleep(Duration::from_secs(1));
                    let is_aborted = *abort.lock().unwrap();
                    if is_aborted {
                        break;
                    }
                    let c = *count.lock().unwrap();
                    before_each(c.clone());
                    let c = c - 1;
                    *count.lock().unwrap() = c.clone();
                    after_each(c);
                }
            }
        });
    }
}
