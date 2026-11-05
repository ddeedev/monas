pub mod theme_color;
use gpui::{App, Global, Pixels, SharedString, black, px, rgb, rgba};

use crate::theme::theme_color::ThemeColor;

#[derive(Clone)]
pub struct Theme {
    // font
    pub font_family: SharedString,
    pub font_size: Pixels,
    pub space_name_font: SharedString,
    pub folder_font: SharedString,
    pub folder_font_size: Pixels,

    pub theme_color: ThemeColor,
}

impl Theme {
    pub fn dark_purple() -> Self {
        Self {
            font_family: "IBM Plex Sans".into(),
            font_size: px(14.),
            space_name_font: "Pacifico".into(),
            folder_font: "IBM Plex Sans".into(),
            folder_font_size: px(14.),
            theme_color: ThemeColor {
                background: rgb(0x636080),
                border: rgb(0x565375),
                text: rgb(0xF8F8F8),
                text_muted: rgb(0xBDBAD4),

                tile_bg: rgb(0x7A769F),
                tile_bg_hover: rgb(0x565375),
                tile_bg_active: rgb(0x8F8BB5),

                button_bg: rgb(0x565375),
                button_hover: rgb(0x7A769F),
                icon_fill: rgb(0x524C73),
                icon_stroke: rgb(0x7A769F),

                essential_bg: rgb(0x7A769F),
                essential_hover: rgb(0x565375),
                essential_active: rgb(0x8F8BB5),

                folder_bg: rgb(0x8F8BB5),
                folder_outline: rgb(0x7A769F),
                folder_hover: rgb(0x565375),
                folder_text: rgb(0xFFFFFF),

                tab_bg: rgb(0x636080),
                tab_hover: rgb(0x565375),
                tab_text: rgb(0xFFFFFF),

                searchbar_bg: rgb(0x6E6A91),
                searchbar_button: rgb(0x7A769F),
                searchbar_text: rgb(0xF8F8F8),
                searchbar_active: rgb(0x3311ff30),
                searchbar_hover: rgb(0x7A769F),
                searchbar_selection: rgba(0x3311ff30),

                sidebar_outline: rgb(0xF8F8F8),
            },
        }
    }
    pub fn pink() -> Self {
        Self {
            font_family: "SYSTEM_UI_FONT".into(),
            font_size: px(14.),
            space_name_font: "Pacifico".into(),
            folder_font: "SYSTEM_UI_FONT".into(),
            folder_font_size: px(14.),
            theme_color: ThemeColor {
                // base
                background: rgb(0xF4B6CF),
                border: rgb(0xD98BAE),
                text: rgb(0x4A1F36),
                text_muted: rgb(0x8A5A74),

                tile_bg: rgb(0xFAD0E1),
                tile_bg_hover: rgb(0xEFA3C3),
                tile_bg_active: rgb(0xE67FAA),

                // buttons
                button_bg: rgb(0xEFA3C3),
                button_hover: rgb(0xE67FAA),
                icon_fill: rgb(0xC9638F),
                icon_stroke: rgb(0x8A3560),

                // essentials
                essential_bg: rgb(0xFAD0E1),
                essential_hover: rgb(0xEFA3C3),
                essential_active: rgb(0xE67FAA),

                // folder & pinned tab
                folder_bg: rgb(0xE67FAA),
                folder_outline: rgb(0x4A1F36),
                folder_hover: rgb(0xEFA3C3),
                folder_text: rgb(0x4A1F36),

                // tab
                tab_bg: rgb(0xF4B6CF),
                tab_hover: rgb(0xEFA3C3),
                tab_text: rgb(0x4A1F36),

                // search bar
                searchbar_bg: rgb(0xFBDCE8),
                searchbar_button: rgb(0xEFA3C3),
                searchbar_text: black().into(),
                searchbar_active: rgba(0xE6458A40),
                searchbar_hover: rgb(0xF7C4D8),
                searchbar_selection: rgba(0x3311ff30),

                sidebar_outline: rgb(0x000000),
            },
        }
    }
}

impl Global for Theme {}

pub fn init(cx: &mut App) {
    cx.set_global(Theme::pink());
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
