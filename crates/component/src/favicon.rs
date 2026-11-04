use gpui::{App, ObjectFit, Pixels, Rgba, SharedString, Window, img, prelude::*, px, svg};

#[derive(IntoElement)]
pub struct Favicon {
    source: SharedString,
    size: Pixels,
    color: Rgba,
}

impl Favicon {
    pub fn new(source: impl Into<SharedString>) -> Self {
        Self {
            source: source.into(),
            size: px(20.),
            color: gpui::rgb(0xFFFFFF),
        }
    }
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }
    pub fn color(mut self, color: Rgba) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Favicon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let (size, color) = (self.size, self.color);

        if self.source.starts_with("http://") || self.source.starts_with("https://") {
            img(self.source)
                .size(size)
                .object_fit(ObjectFit::Contain)
                .with_fallback(move || {
                    svg()
                        .path("icons/globe.svg")
                        .size(size)
                        .text_color(color)
                        .into_any_element()
                })
                .into_any_element()
        } else {
            svg()
                .path(self.source)
                .size(size)
                .text_color(color)
                .into_any_element()
        }
    }
}
