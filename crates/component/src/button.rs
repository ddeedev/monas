use gpui::{
    App, ClickEvent, Div, ElementId, FocusHandle, Interactivity, IntoElement, MouseButton, Pixels,
    Rgba, SharedString, Stateful, StyleRefinement, Styled, Window, div, prelude::*, px, relative,
    rgb, svg,
};
use std::rc::Rc;

type OnClick = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

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
        self.on_click = Some(Rc::new(f));
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

    /// Returns the `FocusHandle` this button should use.
    ///
    /// Two sources, in priority order:
    /// 1. A handle the caller passed in via `.track_focus(&handle)`. Use this when
    ///    the parent needs to control focus (call `handle.focus(window)`, check
    ///    `handle.is_focused(window)`, etc.).
    /// 2. Otherwise, a handle the button owns itself.
    ///
    /// Why `use_keyed_state`: `RenderOnce` components are rebuilt every frame, so
    /// calling `cx.focus_handle()` directly in `render` would create a brand new
    /// handle each frame and focus would be lost on every re-render.
    /// `use_keyed_state` stores the handle in a state entity keyed by `self.id`,
    /// so the same handle is returned on every frame as long as the button keeps
    /// being rendered with the same id.
    ///
    /// NOTE: this means the id must be stable and unique among siblings, or two
    /// buttons will end up sharing one focus handle.
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
            .when_some(self.label.clone(), |this, label| this.aria_label(label))
            // FOCUS: only register this element as focusable when it is both
            // focusable and enabled.
            //  - `tab_index` / `tab_stop` configure the handle: its position in the
            //    Tab order and whether Tab stops on it at all.
            //  - `track_focus` attaches the handle to this element, which puts it in
            //    the window's dispatch tree. That is what enables Tab navigation,
            //    click-to-focus and `is_focused`.
            //  - A disabled button is never tracked, so it can't receive focus.
            .when(self.focusable && !disabled, |this| {
                this.track_focus(
                    &focus_handle
                        .tab_index(self.tab_index)
                        .tab_stop(self.tab_stop),
                )
            })
            // DISABLED look and behavior:
            //  - "not allowed" cursor and dimmed opacity as visual feedback.
            //  - Swallow left mouse-down so it doesn't bubble to a parent that has
            //    its own mouse handler (e.g. a draggable row).
            .when(disabled, |this| {
                this.cursor_not_allowed()
                    .opacity(0.5)
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            })
            .when(!disabled, |this| this.cursor_pointer())
            // CLICK: attach the handler only if the button is enabled AND one was set.
            //  - `(!disabled).then_some(on_click)` -> Option<Option<OnClick>>
            //    (Some(on_click) if enabled, None if disabled)
            //  - `.flatten()` -> Option<OnClick>
            //  - `when_some` then only runs if a handler exists, so a disabled
            //    button never gets a click handler.
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
            .when_some(self.label, |this, label| this.child(label))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        ClickEvent, Context, FocusHandle, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers,
        ParentElement, Render, Styled, TestAppContext, VisualTestContext, div, point,
    };
    use std::cell::Cell;
    use std::rc::Rc;

    struct ButtonHarness {
        disabled: bool,
        focus: FocusHandle,
        click: Rc<Cell<usize>>,
        keyboard_event: Rc<Cell<usize>>,
    }

    impl ButtonHarness {
        fn new(
            disabled: bool,
            click: Rc<Cell<usize>>,
            keyboard_event: Rc<Cell<usize>>,
            cx: &mut Context<Self>,
        ) -> Self {
            Self {
                disabled,
                focus: cx.focus_handle(),
                click,
                keyboard_event,
            }
        }
    }

    impl Render for ButtonHarness {
        fn render(
            &mut self,
            _: &mut gpui::Window,
            _: &mut gpui::prelude::Context<Self>,
        ) -> impl gpui::prelude::IntoElement {
            let click = self.click.clone();
            let keyboard = self.keyboard_event.clone();

            div().size_full().child(
                Button::new("test-btn")
                    .w(px(20.))
                    .h(px(20.))
                    .track_focus(&self.focus)
                    .disabled(self.disabled)
                    .on_click(move |event, _, _| {
                        click.set(click.get() + 1);
                        if matches!(event, ClickEvent::Keyboard(_)) {
                            keyboard.set(keyboard.get() + 1);
                        }
                    }),
            )
        }
    }

    struct SetupResult<'a> {
        view: gpui::Entity<ButtonHarness>,
        vcx: &'a mut VisualTestContext,
        click: Rc<Cell<usize>>,
        keyboard: Rc<Cell<usize>>,
    }

    fn setup(cx: &mut TestAppContext, disabled: bool) -> SetupResult<'_> {
        let click = Rc::new(Cell::new(0));
        let keyboard = Rc::new(Cell::new(0));
        let (c, k) = (click.clone(), keyboard.clone());
        let (view, vcx) = cx.add_window_view(move |_, cx| ButtonHarness::new(disabled, c, k, cx));
        vcx.run_until_parked();
        SetupResult {
            view,
            vcx,
            click,
            keyboard,
        }
    }

    #[gpui::test]
    fn click_fires_handler(cx: &mut TestAppContext) {
        let SetupResult {
            vcx,
            click,
            keyboard,
            ..
        } = setup(cx, false);

        vcx.simulate_click(point(px(10.), px(10.)), Modifiers::default());

        assert_eq!(click.get(), 1);
        assert_eq!(keyboard.get(), 0);
    }

    #[gpui::test]
    fn disabled_click_does_nothing(cx: &mut TestAppContext) {
        let SetupResult { vcx, click, .. } = setup(cx, true);

        vcx.simulate_click(point(px(10.), px(10.)), Modifiers::default());

        assert_eq!(click.get(), 0);
    }

    #[gpui::test]
    fn click_focuses_button(cx: &mut TestAppContext) {
        let SetupResult { view, vcx, .. } = setup(cx, false);

        vcx.simulate_click(point(px(10.), px(10.)), Modifiers::default());

        let focus = view.read_with(vcx, |v, _| v.focus.clone());
        assert!(vcx.update(|window, _| focus.is_focused(window)));
    }

    #[gpui::test]
    fn enter_activates_focused_button(cx: &mut TestAppContext) {
        let SetupResult {
            view,
            vcx,
            click,
            keyboard,
        } = setup(cx, false);

        let focus = view.read_with(vcx, |v, _| v.focus.clone());
        vcx.update(|window, cx| window.focus(&focus, cx));
        let keystroke =
            vcx.update(|_, cx| Keystroke::parse_with_os("enter", cx.platform_info().os).unwrap());
        vcx.simulate_event(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
            prefer_character_input: false,
        });
        vcx.simulate_event(KeyUpEvent { keystroke });
        vcx.run_until_parked();

        assert_eq!(click.get(), 1);
        assert_eq!(keyboard.get(), 1);
    }

    #[gpui::test]
    fn disabled_ignores_enter(cx: &mut TestAppContext) {
        let SetupResult {
            view, vcx, click, ..
        } = setup(cx, false);

        let focus = view.read_with(vcx, |v, _| v.focus.clone());
        vcx.update(|window, cx| window.focus(&focus, cx));
        vcx.simulate_keystrokes("enter");

        assert_eq!(click.get(), 0);
    }
}
