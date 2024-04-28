use crate::{config::DEFAULT_CLEARCOL, mathsutils::lerp4d};

pub struct BackgroundManager {
    start: u64,
    len: u64,
    frame: u64,
    col_start: [f32;4],
    col_end: [f32;4],
    col_current: [f32;4],
    running: bool
}

impl BackgroundManager {
    pub fn new() -> Self {
        Self {
            start: 0,
            len: 0,
            frame: 0,
            col_start: DEFAULT_CLEARCOL,
            col_end: DEFAULT_CLEARCOL,
            col_current: DEFAULT_CLEARCOL,
            running: false
        }
    }
    pub fn start_anim(&mut self, end: [f32;4], len: u64) {
        self.col_end = end;
        self.len = len;
        self.start = self.frame;
        self.running = true;
    }
    pub fn update(&mut self, frame: u64) {
        self.frame = frame;
        if !self.running {
            return
        }

        let n = (self.frame as f32 - self.start as f32) / self.len as f32;
        self.col_current = lerp4d(self.col_start, self.col_end, n);
        if n >= 1.0 {
            self.running = false
        }
    }
    pub fn set_bg(&mut self, col: [f32;4]) {
        // println!("Set to {:?}", col);
        // println!("Backtrace: {}", Backtrace::force_capture());
        self.col_start = col;
        self.col_current = col;
    }
    pub fn current(&self) -> [f32;4] {
        self.col_current
    }
}
