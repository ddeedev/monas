use gpui::{
    App, Entity, IntoElement, MouseButton, RenderOnce, SharedString, Window, div, prelude::*, px,
    rgb, svg,
};
use std::rc::Rc;
use text_input::{Submit, TextInput};

type OnSearch = Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct SearchBar {
    input: Entity<TextInput>,
    on_search: Option<OnSearch>,
}

impl SearchBar {
    pub fn new(input: Entity<TextInput>) -> Self {
        Self {
            input,
            on_search: None,
        }
    }

    pub fn on_search(mut self, f: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_search = Some(Rc::new(f));
        self
    }
}

fn submit(input: &Entity<TextInput>, on_search: &OnSearch, window: &mut Window, cx: &mut App) {
    let text = input.read(cx).content.clone();
    let trimmed = text.trim();
    if !trimmed.is_empty() {
        on_search(&SharedString::from(trimmed.to_string()), window, cx);
    }
}

impl RenderOnce for SearchBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self.input.read(cx).focus_handle.clone();

        let mut root = div()
            .id("search-bar")
            .w_full()
            .h(px(40.0))
            .px(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .bg(rgb(0x7A769F))
            .rounded(px(10.0))
            .border_1()
            .border_color(rgb(0x565375))
            .text_sm()
            .text_color(rgb(0xF8F8F8))
            .cursor_text()
            .on_mouse_down(MouseButton::Left, move |_, window, _cx| {
                window.focus(&focus_handle); // newer gpui: window.focus(&focus_handle, _cx)
            })
            .child(
                svg()
                    .path("icons/search.svg")
                    .size(px(15.0))
                    .flex_shrink_0()
                    .text_color(rgb(0xF8F8F8))
                    .opacity(0.6),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .child(self.input.clone()),
            );

        if let Some(on_search) = self.on_search {
            let (input, f) = (self.input.clone(), on_search.clone());
            root = root.on_action(move |_: &Submit, window, cx| submit(&input, &f, window, cx));

            let (input, f) = (self.input.clone(), on_search);
            root = root.child(
                div()
                    .id("search-submit")
                    .flex_shrink_0()
                    .px(px(12.0))
                    .py(px(5.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .bg(rgb(0x565375))
                    .hover(|s| s.bg(rgb(0x494765)))
                    .text_xs()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child("Search")
                    .on_click(move |_, window, cx| submit(&input, &f, window, cx)),
            );
        }
        root
    }
}
