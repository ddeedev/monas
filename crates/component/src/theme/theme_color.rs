use gpui::Rgba;

#[derive(Clone)]
pub struct ThemeColor {
    // space base color
    pub background: Rgba,
    pub border: Rgba,
    pub text: Rgba,
    pub text_muted: Rgba,

    pub tile_bg: Rgba,
    pub tile_bg_hover: Rgba,
    pub tile_bg_active: Rgba,

    // base botton
    // setting, sidbar, space maangement
    pub button_bg: Rgba,
    pub button_hover: Rgba,

    pub icon_fill: Rgba,
    pub icon_stroke: Rgba,

    // favortte
    pub essential_bg: Rgba,
    pub essential_hover: Rgba,
    pub essential_active: Rgba,

    // folder & pin tab
    pub folder_bg: Rgba,
    pub folder_hover: Rgba,
    pub folder_text: Rgba,

    // tab
    pub tab_bg: Rgba,
    pub tab_hover: Rgba,
    pub tab_text: Rgba,

    // search_bar
    pub searchbar_bg: Rgba,
    pub searchbar_button: Rgba,
    pub searchbar_text: Rgba,
    pub searchbar_active: Rgba,
    pub searchbar_hover: Rgba,
}
