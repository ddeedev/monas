use component::TextBox;
use gpui::{App, Entity, IntoElement, RenderOnce, SharedString, Window, div, prelude::*};
use input::{Submit, TextInput};
use std::rc::Rc;

use super::OnSearch;

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

impl RenderOnce for SearchBar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let input = self.input.clone();

        div()
            .w_full()
            .when_some(self.on_search, move |el, on_search| {
                el.on_action(move |_: &Submit, window, cx| {
                    let text = input.read(cx).content.trim().to_string();
                    if !text.is_empty() {
                        on_search(&SharedString::from(text), window, cx);
                    }
                })
            })
            .child(
                TextBox::new("search-bar", self.input), // .leading_icon("icons/search.svg")
            )
    }
}
