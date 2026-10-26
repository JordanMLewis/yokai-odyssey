use agb::{
    display::{GraphicsFrame, Rgb15},
    input::{Button, ButtonController},
};

pub struct MyGame {
    frames: u32,
    white: bool,
}

impl MyGame {
    pub fn new() -> Self {
        Self { frames: 0, white: false }
    }

    pub fn update(&mut self, input: &ButtonController) {
        self.frames += 1;
        if input.is_just_pressed(Button::A) || self.frames >= 15 {
            self.frames = 0;
            self.white = !self.white;
        }
    }

    pub fn backdrop(&self) -> Rgb15 {
        if self.white { Rgb15::WHITE } else { Rgb15::BLACK }
    }

    pub fn show(&mut self, _frame: &mut GraphicsFrame<'_>) {}
}