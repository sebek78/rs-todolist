pub struct AppState {
    pub menu_index: u32,
    max_index: u32,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            menu_index: 0,
            max_index: 1,
        }
    }
    pub fn menu_next(&mut self) {
        if self.menu_index < self.max_index {
            self.menu_index += 1;
        }
    }
    pub fn menu_prev(&mut self) {
        if self.menu_index > 0 {
            self.menu_index -= 1;
        }
    }
}
