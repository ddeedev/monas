use crate::theme::ActiveTheme;
use gpui::{
    AnyElement, App, ElementId, Entity, MouseButton, Pixels, SharedString, Window, div, prelude::*,
    px, svg,
};
use input::TextInput;

#[derive(IntoElement)]
pub struct TextBox {
    id: ElementId,
    input: Entity<TextInput>,
    height: Pixels,
    leading_icon: Option<SharedString>,
    trailing: Option<AnyElement>,
}

impl TextBox {
    pub fn new(id: impl Into<ElementId>, input: Entity<TextInput>) -> Self {
        Self {
            id: id.into(),
            input,
            height: px(40.),
            leading_icon: None,
            trailing: None,
        }
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.height = height;
        self
    }

    /// Path to a bundled svg, e.g. "icons/search.svg"
    pub fn leading_icon(mut self, path: impl Into<SharedString>) -> Self {
        self.leading_icon = Some(path.into());
        self
    }

    /// Anything shown on the right: a button, a clear icon, a shortcut hint.
    pub fn trailing(mut self, el: impl IntoElement) -> Self {
        self.trailing = Some(el.into_any_element());
        self
    }
}

impl RenderOnce for TextBox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        let focused = focus_handle.is_focused(window);

        let c = &cx.theme().theme_color;
        let (bg, border, text, muted) = (c.searchbar_bg, c.border, c.searchbar_text, c.text_muted);
        let focus_border = c.searchbar_selection;

        div()
            .id(self.id)
            .w_full()
            .h(self.height)
            .px(px(12.))
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .bg(bg)
            .rounded(px(10.))
            .border_1()
            .border_color(if focused { focus_border } else { border })
            .text_sm()
            .text_color(text)
            .cursor_text()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                window.focus(&focus_handle, cx);
            })
            .when_some(self.leading_icon, |el, path| {
                el.child(
                    svg()
                        .path(path)
                        .size(px(15.))
                        .flex_shrink_0()
                        .text_color(muted),
                )
            })
            .child(div().flex_1().min_w_0().overflow_hidden().child(self.input))
            .children(self.trailing)
    }
}
