use agb::{
    display::{GraphicsFrame, Rgb, Rgb15},
    input::{Button, ButtonController},
};

const HUE_MAX: u16 = 1536;
const HUE_STEP: u16 = 16; // raise for a faster cycle, lower for a slower one

pub struct MyGame {
    hue: u16,
}

impl MyGame {
    pub fn new() -> Self {
        Self { hue: 0 }
    }

    pub fn update(&mut self, input: &ButtonController) {
        if input.is_pressed(Button::A) {
            self.hue = (self.hue + HUE_STEP) % HUE_MAX;
        }
    }

    pub fn backdrop(&self) -> Rgb15 {
        let x = (self.hue % 256) as u8;
        let (r, g, b) = match self.hue / 256 {
            0 => (255, x, 0),
            1 => (255 - x, 255, 0),
            2 => (0, 255, x),
            3 => (0, 255 - x, 255),
            4 => (x, 0, 255),
            _ => (255, 0, 255 - x),
        };
        Rgb::new(r, g, b).to_rgb15()
    }

    pub fn show(&mut self, _frame: &mut GraphicsFrame<'_>) {}
}
