use agb::input::{Button, ButtonController};

use agb::display::{
    GraphicsFrame, Palette16, Rgb, Rgb15,
    font::{AlignmentKind, Font, Layout, LayoutSettings, ObjectTextRenderer},
    object::Size,
};

static FONT: Font = agb::include_font!("assets/pixelta.ttf", 18);

static TEXT_PALETTE: &Palette16 = {
    let mut colors = [Rgb15::BLACK; 16];
    colors[1] = Rgb15::WHITE;
    &Palette16::new(colors)
};

fn text_sprite_size(font: &Font) -> (Size, i32) {
    match font.line_height() {
        0..=8 => (Size::S32x8, 32),
        9..=16 => (Size::S32x16, 32),
        17..=32 => (Size::S32x32, 32),
        33..=64 => (Size::S64x64, 64),
        _ => panic!("font line height exceeds the maximum supported text sprite size"),
    }
}

const HUE_MAX: u16 = 1536;
const HUE_STEP: u16 = 32; // raise for a faster cycle, lower for a slower one

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

    pub fn show(&mut self, frame: &mut GraphicsFrame<'_>) {
        let line_height = FONT.line_height();
        let (sprite_size, max_group_width) = text_sprite_size(&FONT);
        let layout = Layout::new(
            "Hold A to change the background color",
            &FONT,
            &LayoutSettings::new()
                .with_alignment(AlignmentKind::Centre)
                .with_max_line_length(agb::display::WIDTH)
                .with_max_group_width(max_group_width),
        );

        let renderer = ObjectTextRenderer::new(TEXT_PALETTE.into(), sprite_size);
        for group in layout {
            let text = renderer.show(&group, (0, (agb::display::HEIGHT - line_height) / 2));
            text.show(frame);
        }
    }
}
