pub struct Aimbot {
    sensitivity: f32,
    target_lock: bool,
}

impl Aimbot {
    pub fn new(sensitivity: f32) -> Self {
        Aimbot {
            sensitivity,
            target_lock: false,
        }
    }

    pub fn activate(&mut self) {
        self.target_lock = true;
    }

    pub fn deactivate(&mut self) {
        self.target_lock = false;
    }
}