pub struct Wallhack {
    enabled: bool,
}

impl Wallhack {
    pub fn new() -> Self {
        Wallhack {
            enabled: false,
        }
    }

    pub fn activate(&mut self) {
        self.enabled = true;
    }

    pub fn deactivate(&mut self) {
        self.enabled = false;
    }
}