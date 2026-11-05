use gpui::{
    App, ClickEvent, Div, ElementId, FocusHandle, Interactivity, IntoElement, MouseButton, Pixels,
    Rgba, SharedString, Stateful, StyleRefinement, Styled, Window, div, prelude::*, px, relative,
    rgb, svg,
};

type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

// DEFAULT VALUE
/// ICON
const ICON_SIZE_H: Pixels = px(16.);
const ICON_SIZE_W: Pixels = px(18.);

/// Reusable button accross this project
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    base: Stateful<Div>,
    selected: bool,
    label: Option<SharedString>,
    icon: Option<SharedString>,
    icon_w: Option<Pixels>,
    icon_h: Option<Pixels>,
    icon_color: Option<Rgba>,
    icon_hover_color: Option<Rgba>,
    disabled: bool,
    on_click: Option<OnClick>,
    provided_focus_handle: Option<FocusHandle>,
    tab_index: isize,
    tab_stop: bool,
    focusable: bool,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();

        Self {
            base: div().id(id.clone()),
            id,
            selected: false,
            label: None,
            icon: None,
            icon_w: None,
            icon_h: None,
            icon_color: None,
            icon_hover_color: None,
            disabled: false,
            on_click: None,
            provided_focus_handle: None,
            focusable: true,
            tab_index: 0,
            tab_stop: true,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selectd: bool) -> Self {
        self.selected = selectd;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn icon_h(mut self, icon_h: Pixels) -> Self {
        self.icon_h = Some(icon_h);
        self
    }

    pub fn icon_w(mut self, icon_w: Pixels) -> Self {
        self.icon_w = Some(icon_w);
        self
    }

    pub fn icon_color(mut self, icon_color: Rgba) -> Self {
        self.icon_color = Some(icon_color);
        self
    }

    pub fn icon_hover_color(mut self, icon_hover_color: Rgba) -> Self {
        self.icon_hover_color = Some(icon_hover_color);
        self
    }

    pub fn track_focus(mut self, focus_handle: &FocusHandle) -> Self {
        self.provided_focus_handle = Some(focus_handle.clone());
        self
    }

    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = tab_index;
        self
    }

    pub fn tab_stop(mut self, tab_stop: bool) -> Self {
        self.tab_stop = tab_stop;
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    fn focus_handle(&self, window: &mut Window, cx: &mut App) -> FocusHandle {
        self.provided_focus_handle.clone().unwrap_or_else(|| {
            window
                .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
                .read(cx)
                .clone()
        })
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Button {}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let defalut_icon_color: Rgba = rgb(0xF8F8F8);

        let focus_handle = self.focus_handle(window, cx);
        let disabled = self.disabled;
        let on_click = self.on_click;

        self.base
            .flex()
            .justify_center()
            .items_center()
            .line_height(relative(1.))
            .when_some(self.label, |this, label| this.aria_label(label))
            .when(
                self.focusable && (disabled || focus_handle.is_focused(window)),
                |this| {
                    this.track_focus(
                        &focus_handle
                            .tab_index(self.tab_index)
                            .tab_stop(self.tab_stop && !disabled),
                    )
                },
            )
            .when(disabled, |this| {
                this.on_mouse_down(MouseButton::Left, |_, _, cx| {
                    cx.stop_propagation();
                })
            })
            .when_some(
                (!disabled).then_some(on_click).flatten(),
                |this, on_click| {
                    this.on_click(move |event, window, cx| on_click(event, window, cx))
                },
            )
            .when_some(self.icon, |this, path| {
                let default_color = self.icon_color.unwrap_or(defalut_icon_color);
                let hover_color = self.icon_hover_color;
                this.child(
                    svg()
                        .path(path)
                        .flex_none()
                        .w(self.icon_w.unwrap_or(ICON_SIZE_W))
                        .h(self.icon_h.unwrap_or(ICON_SIZE_H))
                        .text_color(self.icon_color.unwrap_or(default_color))
                        .when_some(hover_color, |svg_el, h_color| {
                            svg_el.hover(move |style| style.text_color(h_color))
                        }),
                )
            })
    }
}
