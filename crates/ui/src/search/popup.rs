use context::BrowserData;
use gpui::{
    App, Context, Entity, EventEmitter, IntoElement, KeyBinding, MouseButton, Render, SharedString,
    Window, actions, deferred, div, prelude::*, px, rgb, rgba, svg,
};
use input::{Submit, TextInput};

actions!(popup, [SelectPrev, SelectNext, Dismiss]);

pub enum PopupEvent {
    Search(SharedString),
    Open(BrowserData),
    Dismiss,
}

impl EventEmitter<PopupEvent> for Popup {}

pub struct Popup {
    pub input: Entity<TextInput>,
    pub items: Vec<BrowserData>,
    selected: usize,
    visible: bool,
}

impl Popup {
    pub fn bind_keys(cx: &mut App) {
        let ctx = Some("Popup");
        cx.bind_keys([
            KeyBinding::new("up", SelectPrev, ctx),
            KeyBinding::new("down", SelectNext, ctx),
            KeyBinding::new("ctrl-p", SelectPrev, ctx),
            KeyBinding::new("ctrl-n", SelectNext, ctx),
            KeyBinding::new("escape", Dismiss, ctx),
        ]);
    }

    pub fn new(input: Entity<TextInput>, items: Vec<BrowserData>, cx: &mut Context<Self>) -> Self {
        cx.observe(&input, |this, _, cx| {
            this.selected = 0;
            cx.notify();
        })
        .detach();
        Self {
            input,
            items,
            selected: 0,
            visible: false,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.visible = true;
        self.selected = 0;
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.focus(&focus_handle, cx);
        cx.notify();
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.visible = false;
        cx.notify();
    }

    pub fn focus(&self, window: &mut Window, cx: &mut Context<Self>) {
        cx.observe(&self.input, |this, _, cx| {
            this.selected = 0;
            cx.notify();
        })
        .detach();
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.focus(&focus_handle, cx);
    }

    fn filtered(&self, cx: &App) -> Vec<BrowserData> {
        let q = self.input.read(cx).content.to_lowercase();
        self.items
            .iter()
            .filter(|i| q.is_empty() || i.url.to_lowercase().contains(&q))
            .cloned()
            .collect()
    }

    fn select_next(&mut self, _: &SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let len = self.filtered(cx).len();
        if len > 0 {
            self.selected = (self.selected + 1) % len;
            cx.notify();
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, _: &mut Window, cx: &mut Context<Self>) {
        let len = self.filtered(cx).len();
        if len > 0 {
            self.selected = (self.selected + len - 1) % len;
            cx.notify();
        }
    }

    fn dismiss(&mut self, _: &Dismiss, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(PopupEvent::Dismiss);
    }

    // Enter: open the highlighted row, or search the raw text if nothing matches.
    fn confirm(&mut self, _: &Submit, _: &mut Window, cx: &mut Context<Self>) {
        let rows = self.filtered(cx);
        if let Some(item) = rows.get(self.selected) {
            cx.emit(PopupEvent::Open(item.clone()));
        } else {
            let text = self.input.read(cx).content.trim().to_string();
            if !text.is_empty() {
                cx.emit(PopupEvent::Search(text.into()));
            }
        }
    }
}

impl Render for Popup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.visible {
            return div().into_any_element();
        }

        let rows = self.filtered(cx);
        let selected = self.selected.min(rows.len().saturating_sub(1));

        let list = rows.into_iter().enumerate().map(|(i, item)| {
            let is_sel = i == selected;
            let open = item.clone();
            div()
                .id(("popup-row", i))
                .h(px(44.))
                .px_2()
                .rounded(px(8.))
                .flex()
                .flex_row()
                .items_center()
                .gap_3()
                .cursor_pointer()
                .when(is_sel, |el| el.bg(rgb(0x3D3DF5)))
                .on_mouse_move(cx.listener(move |this, _, _, cx| {
                    if this.selected != i {
                        this.selected = i;
                        cx.notify();
                    }
                }))
                .on_click(cx.listener(move |_, _, _, cx| {
                    cx.emit(PopupEvent::Open(open.clone()));
                }))
                .child(
                    svg()
                        .path(item.favicon.clone())
                        .size(px(20.))
                        .text_color(rgb(0xF8F8F8)),
                )
                .child(
                    div()
                        .flex_1()
                        .text_sm()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child(SharedString::from(item.url.clone())),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xF8F8F8))
                        .opacity(if is_sel { 1.0 } else { 0.4 })
                        .child("Switch to Tab  →"),
                )
        });

        // Full-window overlay: dimmed backdrop + centered panel.
        deferred(
            div()
                .id("popup-backdrop")
                .flex()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .items_center()
                .justify_center()
                .bg(rgba(0x00000066))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|_, _, _, cx| cx.emit(PopupEvent::Dismiss)),
                )
                .child(
                    div()
                        .id("popup-panel")
                        .key_context("Popup")
                        .occlude() // clicks on the panel don't reach the backdrop
                        .on_action(cx.listener(Self::select_next))
                        .on_action(cx.listener(Self::select_prev))
                        .on_action(cx.listener(Self::dismiss))
                        .on_action(cx.listener(Self::confirm))
                        .w(px(760.))
                        .h(px(360.))
                        .max_w_full()
                        .flex()
                        .flex_col()
                        .bg(rgb(0x1C1C20))
                        .border_1()
                        .border_color(rgba(0xFFFFFF1A))
                        .rounded(px(12.))
                        .shadow_2xl()
                        .text_color(rgb(0xF8F8F8))
                        .child(
                            div()
                                .h(px(56.))
                                .px_4()
                                .flex()
                                .items_center()
                                .gap_3()
                                .text_lg()
                                .border_b_1()
                                .border_color(rgba(0xFFFFFF1A))
                                .child(svg().path("icons/search.svg").size(px(16.)).opacity(0.6))
                                .child(div().flex_1().child(self.input.clone())),
                        )
                        .child(div().p_2().flex().flex_col().gap_1().children(list)),
                ),
        )
        .into_any_element()
    }
}
