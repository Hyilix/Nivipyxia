use winit::dpi;

pub struct Mouse {
    current_pos: dpi::PhysicalPosition<f64>,
    last_pos: dpi::PhysicalPosition<f64>,
    pressed: bool,
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            current_pos: dpi::PhysicalPosition::new(0f64, 0f64),
            last_pos: dpi::PhysicalPosition::new(0f64, 0f64),
            pressed: false
        }
    }
}

impl Mouse {
    // Update the position of the mouse
    pub fn update_position(&mut self, pos: dpi::PhysicalPosition<f64>) {
        self.last_pos = self.current_pos;
        self.current_pos = pos;
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }

    pub fn toggle_pressed(&mut self) {
        self.pressed = !self.pressed;
    }
}

impl Mouse {
    pub fn get_tile_pos() {

    }
}
