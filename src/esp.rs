pub struct ESP {
    visibility: bool,
}

impl ESP {
    pub fn new() -> Self {
        ESP {
            visibility: false,
        }
    }

    pub fn activate(&mut self) {
        self.visibility = true;
    }

    pub fn deactivate(&mut self) {
        self.visibility = false;
    }
}