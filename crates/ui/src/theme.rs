use gpui::{App, Global, Pixels, Rgba, SharedString, prelude::*, px, rgb};

#[derive(Clone)]
pub struct Theme {
    // font
    pub font_family: SharedString,
    pub font_size: Pixels,

    pub sidebar_bg: Rgba,
    pub content_bg: Rgba,
    pub content_border: Rgba,
    pub tile_bg: Rgba,
    pub tile_bg_hover: Rgba,
    pub tile_bg_active: Rgba,

    pub text: Rgba,
    pub text_muted: Rgba,
}

impl Theme {
    pub fn dark_purple() -> Self {
        Self {
            font_family: ".SystemUIFont".into(),
            font_size: px(14.),

            sidebar_bg: rgb(0x636080),
            content_bg: rgb(0x7A769F),
            content_border: rgb(0x565375),
            tile_bg: rgb(0x7A769F),
            tile_bg_hover: rgb(0x565375),
            tile_bg_active: rgb(0xD3D3D3),

            text: rgb(0xF8F8F8),
            text_muted: rgb(0xBDBAD4),
        }
    }
}

impl Global for Theme {}

pub fn init(cx: &mut App) {
    cx.set_global(Theme::dark_purple());
}

// write thene anywhere
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        self.global::<Theme>()
    }
}
