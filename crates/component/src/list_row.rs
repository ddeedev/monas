use crate::theme::ActiveTheme;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, SharedString, Window, div, prelude::*, px,
};

type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct ListRow {
    id: ElementId,
    label: SharedString,
    depth: usize,
    active: bool,
    leading: Option<AnyElement>,
    on_click: Option<OnClick>,
}

impl ListRow {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            depth: 0,
            active: false,
            leading: None,
            on_click: None,
        }
    }
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
    pub fn leading(mut self, el: impl IntoElement) -> Self {
        self.leading = Some(el.into_any_element());
        self
    }
    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl RenderOnce for ListRow {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let (hover, active, text) = (
            theme.theme_color.tile_bg_hover,
            theme.theme_color.tile_bg_active,
            theme.theme_color.text,
        );
        let font_size = theme.folder_font_size;
        let indent = 8.0 + self.depth as f32 * 8.0;

        div()
            .id(self.id)
            .h(px(40.))
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .pl_2()
            .when(self.depth > 0, |el| el.ml(px(indent)))
            .rounded(px(10.))
            .cursor_pointer()
            .when(self.active, move |el| el.bg(active))
            .hover(move |s| s.bg(hover))
            .when_some(self.on_click, |el, f| el.on_click(f))
            .when_some(self.leading, |el, leading| {
                el.child(
                    div()
                        .h(px(25.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(leading),
                )
            })
            .child(
                div()
                    .text_size(font_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text)
                    .child(self.label),
            )
    }
}
