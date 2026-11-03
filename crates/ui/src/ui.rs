pub mod platform;
pub mod search;
pub mod sidebar;
pub mod theme;

pub use theme::{ActiveTheme, Theme};

use gpui::{AnyElement, IntoElement, ObjectFit, img, prelude::*, px, rgb, svg};

fn favicon_element(favicon: &str, fallback_color: u32) -> AnyElement {
    if favicon.starts_with("http://") || favicon.starts_with("https://") {
        img(favicon.to_string())
            .size(px(20.))
            .object_fit(ObjectFit::Contain)
            .with_fallback(move || {
                svg()
                    .path("icons/globe.svg")
                    .size(px(20.))
                    .text_color(rgb(fallback_color))
                    .into_any_element()
            })
            .into_any_element()
    } else {
        svg()
            .path(favicon.to_string())
            .size(px(20.))
            .text_color(rgb(fallback_color))
            .into_any_element()
    }
}
