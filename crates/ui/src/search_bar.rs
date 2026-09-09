use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb, svg};

type OnSearch = Box<dyn Fn(&mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct SearchBar {
    placeholder: SharedString,
    on_search: Option<OnSearch>,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            placeholder: "search".into(),
            on_search: None,
        }
    }

    pub fn on_search(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_search = Some(Box::new(f));
        self
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SearchBar {
    fn render(mut self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_search = self.on_search.take();

        div()
            .relative()
            .w_full()
            .child(
                // search icon
                div()
                    .absolute()
                    .left_0()
                    .top_0()
                    .bottom_0()
                    .flex()
                    .items_center()
                    .pl_3()
                    .child(
                        svg()
                            .path("icons/search.svg")
                            .size(px(15.0))
                            .text_color(rgb(0xF8F8F8))
                            .opacity(0.5),
                    ),
            )
            .child(
                // input surface — matches workspace-card styling
                div()
                    .id("search-input")
                    .w_full()
                    .h(px(40.0))
                    .pl_3()
                    //.pl(px(34.0))
                    //.pr(px(78.0))
                    .flex()
                    .items_center()
                    .bg(rgb(0x7A769F))
                    .rounded(px(6.0))
                    .text_sm()
                    .child(
                        div()
                            .text_color(rgb(0xF8F8F8))
                            .opacity(0.5)
                            .child(self.placeholder.clone()),
                    ),
            )
            .child({
                let mut btn = div()
                    .id("search-submit")
                    .absolute()
                    .right(px(5.0))
                    .top(px(5.0))
                    .bottom(px(5.0))
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(5.0))
                    .cursor_pointer()
                    .bg(rgb(0x565375))
                    .hover(|s| s.bg(rgb(0x494765)))
                    .text_color(rgb(0xF8F8F8))
                    .text_xs()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child("Search");

                if let Some(on_search) = on_search {
                    btn = btn.on_click(move |_, window, cx| {
                        on_search(window, cx);
                    });
                }
                btn
            })
    }
}
