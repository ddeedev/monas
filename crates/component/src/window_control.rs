use gpui::{
    App, Div, ElementId, IntoElement, Rgba, SharedString, Stateful, Window, div, prelude::*, px,
    rgb,
};

use crate::theme::ActiveTheme;

#[derive(IntoElement)]
pub struct WindowControl {
    group: SharedString,
    base: Stateful<Div>,
    idle: Option<Rgba>,
    stroke: Option<Rgba>,
}

impl WindowControl {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        let group: SharedString = id.to_string().into();
        Self {
            base: div().id(id).group(group.clone()),
            group,
            idle: None,
            stroke: None,
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
        let theme = cx.theme();
        let hover_border = theme.theme_color.background;
        let stroke = self.stroke.unwrap_or(theme.theme_color.tile_bg_active);
        let idle = self.idle;
        let group = self.group;

        let light = move |name: &'static str, hover_color: u32| {
            div()
                .id(SharedString::from(format!("{group}_{name}")))
                .size(px(16.))
                .border_1()
                .border_color(stroke)
                .rounded_full()
                .cursor_pointer()
                .when_some(idle, |this, color| this.bg(color))
                .group_hover(group.clone(), move |s| {
                    s.bg(rgb(hover_color)).border_color(hover_border)
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
