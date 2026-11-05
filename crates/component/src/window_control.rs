use gpui::{
    App, ClickEvent, Div, ElementId, FocusHandle, Interactivity, IntoElement, MouseButton, Pixels,
    Rgba, SharedString, Stateful, StyleRefinement, Styled, Window, div, prelude::*, px, relative,
    rgb, svg,
};

use crate::theme::ActiveTheme;

#[derive(IntoElement)]
pub struct WindowControl {
    grop: SharedString,
    base: Stateful<Div>,
    idle: Option<Rgba>,
    stroke: Option<Rgba>,
}

impl WindowControl {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            base: div().id(id.clone()).group(id.to_string()),
            idle: None,
            stroke: None,
            grop: id.to_string().into(),
        }
    }

    pub fn idle(mut self, idle: Rgba) -> Self {
        self.idle = Some(idle);
        self
    }

    pub fn stroke(mut self, stroke: Rgba) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl RenderOnce for WindowControl {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = &cx.theme().clone();
        let idle = t.theme_color.background;

        let light = |id: &'static str, hover_color: u32| {
            div()
                .id(id)
                .size(px(16.))
                .border_1()
                .border_color(self.stroke.unwrap_or(t.theme_color.tile_bg_active))
                .rounded_full()
                .cursor_pointer()
                .when_some(self.idle, |this, color| this.bg(color))
                .group_hover(self.grop.clone(), move |s| {
                    s.bg(rgb(hover_color)).border_color(idle)
                })
        };

        self.base
            .flex()
            .items_center()
            .gap_2()
            .pl_3()
            .child(
                light("traffic-light-close", 0xFF5F57)
                    .on_click(|_, window, _| window.remove_window()),
            )
            .child(
                light("traffic-light-minimize", 0xFEBC2E)
                    .on_click(|_, window, _| window.minimize_window()),
            )
            .child(
                light("traffic-light-zoom", 0x28C840).on_click(|_, window, _| window.zoom_window()),
            )
    }
}
