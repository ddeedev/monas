use crate::theme::ActiveTheme;
use gpui::{AnyElement, App, ElementId, Window, div, prelude::*, px, relative};

#[derive(IntoElement)]
pub struct Tile {
    id: ElementId,
    width: f32, // fraction of the row, e.g. 0.31
    child: Option<AnyElement>,
}

impl Tile {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            width: 1.0,
            child: None,
        }
    }
    pub fn width(mut self, fraction: f32) -> Self {
        self.width = fraction;
        self
    }
    pub fn child(mut self, el: impl IntoElement) -> Self {
        self.child = Some(el.into_any_element());
        self
    }
}

impl RenderOnce for Tile {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let (bg, hover) = (
            cx.theme().theme_color.tile_bg,
            cx.theme().theme_color.tile_bg_hover,
        );
        div()
            .id(self.id)
            .w(relative(self.width))
            .h(px(45.))
            .bg(bg)
            .rounded(px(10.))
            .cursor_pointer()
            .flex()
            .justify_center()
            .items_center()
            .hover(move |s| s.bg(hover))
            .children(self.child)
    }
}
