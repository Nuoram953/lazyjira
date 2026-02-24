pub struct LoadingSpinner {
    frames: &'static [&'static str],
    pub index: usize,
}

impl LoadingSpinner {
    pub fn new() -> Self {
        Self {
            frames: &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            index: 0,
        }
    }

    // pub fn next_frame(&mut self) -> &str {
    //     let frame = self.frames[self.index % self.frames.len()];
    //     self.index = self.index.wrapping_add(1);
    //     frame
    // }

    pub fn current_frame(&self) -> &str {
        self.frames[self.index % self.frames.len()]
    }

    pub fn advance(&mut self) {
        self.index = self.index.wrapping_add(1);
    }

    // pub fn reset(&mut self) {
    //     self.index = 0;
    // }
}

impl Default for LoadingSpinner {
    fn default() -> Self {
        Self::new()
    }
}
