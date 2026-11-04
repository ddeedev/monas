pub mod app_sidebar;
pub mod event;

use crate::platform::set_traffic_lights_hidden;
use app_sidebar::AppSidebar;
use event::SidebarResizeDrag;

use context::space::SidebarContext;
use gpui::{App, Entity, IntoElement, SharedString, Window, deferred, div, prelude::*, px};
use std::{fmt::Debug, time::Duration};

use crate::ActiveTheme;
use input::TextInput;
use std::rc::Rc;

pub enum SidebarEvent {
    Search(SharedString),
}
impl gpui::EventEmitter<SidebarEvent> for SidebarView {}

type OnSearch = Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>;

#[derive(Debug, Clone)]
pub struct SidebarView {
    hidden: bool,
    width: f32,
    width_anim: f32,
    animating: bool,
    floating_visible: bool,
    floating_progress: f32,
    floating_animating: bool,
    space_name: SharedString,
    entity: Entity<SidebarContext>,
    space_logos: Vec<String>,
    active_space: usize,
    search_input: Entity<TextInput>,
}

impl SidebarView {
    const DEFAULT_WIDTH: f32 = 242.0;

    pub fn new(
        space_name: SharedString,
        active_space: usize,
        entity: Entity<SidebarContext>,
        search_input: Entity<TextInput>,
    ) -> Self {
        Self {
            hidden: false,
            width: 242.0,
            width_anim: 242.0,
            animating: false,
            floating_visible: false,
            floating_progress: 0.0,
            floating_animating: false,
            space_logos: vec![],
            active_space,
            space_name,
            entity,
            search_input,
        }
    }

    pub fn register_logos(&mut self, cx: &mut gpui::Context<Self>, space_logos: Vec<String>) {
        if self.space_logos == space_logos {
            return;
        }
        self.space_logos = space_logos;
        cx.notify();
    }

    pub fn set_active_space(&mut self, cx: &mut gpui::Context<Self>, active_space: usize) {
        if self.active_space == active_space {
            return;
        }
        self.active_space = active_space;
        cx.notify();
    }

    pub fn reset(&mut self, cx: &mut gpui::Context<Self>) {
        self.hidden = false;
        self.width = Self::DEFAULT_WIDTH;
        self.width_anim = Self::DEFAULT_WIDTH;
        self.animating = false;
        self.floating_visible = false;
        self.floating_progress = 0.0;
        self.floating_animating = false;

        cx.notify();
    }

    // 1.0 when fully open, 0.0 when fully hidden (animated).
    pub fn visible_fraction(&self) -> f32 {
        self.width_anim / self.width
    }

    pub fn toggle(&mut self, cx: &mut gpui::Context<Self>) {
        let hidden = self.hidden;
        if hidden && self.floating_visible {
            self.hide_floating_immediately(cx);
            self.show_docked_immediately(cx);
        } else {
            self.set_floating_visible(false, cx);
            self.set_hidden(!hidden, cx);
        }
    }

    // Hide the floating sidebar instantly, without the slide-out animation.
    fn hide_floating_immediately(&mut self, cx: &mut gpui::Context<Self>) {
        self.floating_visible = false;
        self.floating_progress = 0.0;
        cx.notify();
    }

    // Show the docked sidebar instantly, without the slide-in animation.
    fn show_docked_immediately(&mut self, cx: &mut gpui::Context<Self>) {
        self.hidden = false;
        self.width_anim = self.width;
        cx.notify();
    }

    pub fn set_hidden(&mut self, hidden: bool, cx: &mut gpui::Context<Self>) {
        if self.hidden == hidden {
            return;
        }
        self.hidden = hidden;
        cx.notify();
        if self.animating {
            return;
        }
        self.animating = true;
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
                match this.update(cx, |this, cx| {
                    let target = if this.hidden { 0.0 } else { this.width };
                    let diff = target - this.width_anim;
                    let settled = diff.abs() < 3.0;
                    this.width_anim = if settled {
                        target
                    } else {
                        this.width_anim + diff * 0.65
                    };
                    cx.notify();
                    settled
                }) {
                    Ok(true) | Err(_) => break,
                    Ok(false) => {}
                }
            }
            let _ = this.update(cx, |this, _| this.animating = false);
        })
        .detach();
    }

    pub fn set_floating_visible(&mut self, visible: bool, cx: &mut gpui::Context<Self>) {
        if self.floating_visible == visible {
            return;
        }
        self.floating_visible = visible;
        cx.notify();
        if self.floating_animating {
            return;
        }
        self.floating_animating = true;
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
                match this.update(cx, |this, cx| {
                    let target = if this.floating_visible { 1.0 } else { 0.0 };
                    let diff = target - this.floating_progress;
                    let settled = diff.abs() < 0.03;
                    this.floating_progress = if settled {
                        target
                    } else {
                        this.floating_progress + diff * 0.5
                    };
                    cx.notify();
                    settled
                }) {
                    Ok(true) | Err(_) => break,
                    Ok(false) => {}
                }
            }
            let _ = this.update(cx, |this, _| this.floating_animating = false);
        })
        .detach();
    }
}

impl Render for SidebarView {
    fn render(&mut self, window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        // NOTE: 1 @ddeedev
        // 1.1 Disable native traffic light
        // Then using custom traffic light later
        set_traffic_lights_hidden(window, true);

        let toggle = cx.listener(|this, _: &gpui::ClickEvent, _window, cx| this.toggle(cx));

        let toggle_floating = cx.listener(|this, _: &gpui::ClickEvent, _window, cx| {
            this.hide_floating_immediately(cx);
            this.show_docked_immediately(cx);
        });

        let show_floating = cx.listener(|this, _: &gpui::MouseMoveEvent, _window, cx| {
            if this.hidden {
                this.set_floating_visible(true, cx);
            }
        });

        let entity = cx.entity();

        let view = cx.entity();
        let on_search: OnSearch = Rc::new(move |q, _w, cx| {
            view.update(cx, |_, cx| cx.emit(SidebarEvent::Search(q.clone())));
        });

        let theme = cx.theme().clone();

        let (tile,) = (theme.theme_color.tile_bg_active,);

        div()
            .h_full()
            .flex_shrink_0()
            .flex()
            .flex_row()
            .on_drag_move(
                move |event: &gpui::DragMoveEvent<SidebarResizeDrag>, _window, cx| {
                    let pos: f32 = event.event.position.x.into();
                    let new_width = pos.clamp(200.0, 500.0);
                    entity.update(cx, |view, cx| {
                        view.width = new_width;
                        if !view.hidden {
                            view.width_anim = new_width;
                        }
                        cx.notify();
                    });
                },
            )
            .when(self.width_anim > 0.0, |el| {
                el.child(
                    div()
                        .w(px(self.width_anim))
                        .h_full()
                        .flex_shrink_0()
                        .overflow_hidden()
                        .flex()
                        .justify_end()
                        .child(
                            AppSidebar::new(
                                false,
                                self.space_name.clone(),
                                self.active_space,
                                self.entity.clone(),
                                self.space_logos.clone(),
                                self.search_input.clone(),
                            )
                            .on_search({
                                let f = on_search.clone();
                                move |q, w, cx| f(q, w, cx)
                            })
                            .width(self.width)
                            .toggle_sidebar(move |window, cx| {
                                toggle(&gpui::ClickEvent::default(), window, cx);
                            }),
                        ),
                )
                .child(
                    div()
                        .id("sidebar-resize-handle")
                        .w(px(3.0))
                        .h_full()
                        .cursor_col_resize()
                        .hover(move |s| s.bg(tile))
                        .on_drag(SidebarResizeDrag, |_, _, _, cx| {
                            cx.new(|_| SidebarResizeDrag)
                        }),
                )
            })
            .child(deferred(
                div()
                    .when(self.hidden && self.width_anim <= 0.0, |el| {
                        el.child(
                            div()
                                .id("sidebar-hover-zone")
                                .absolute()
                                .left_0()
                                .top_0()
                                .bottom_0()
                                .w(px(12.0))
                                .on_mouse_move(show_floating),
                        )
                    })
                    .when(
                        self.floating_visible || self.floating_progress > 0.0,
                        |el| {
                            el.child(
                                div()
                                    .id("floating-sidebar")
                                    .absolute()
                                    .left(px(-(1.0 - self.floating_progress) * self.width))
                                    .top_0()
                                    .bottom_0()
                                    .occlude()
                                    .shadow_lg()
                                    .child(
                                        AppSidebar::new(
                                            false,
                                            self.space_name.clone(),
                                            self.active_space,
                                            self.entity.clone(),
                                            self.space_logos.clone(),
                                            self.search_input.clone(),
                                        )
                                        .on_search({
                                            let f = on_search.clone();
                                            move |q, w, cx| f(q, w, cx)
                                        })
                                        .width(self.width)
                                        .toggle_sidebar(
                                            move |window, cx| {
                                                toggle_floating(
                                                    &gpui::ClickEvent::default(),
                                                    window,
                                                    cx,
                                                );
                                            },
                                        ),
                                    ),
                            )
                        },
                    ),
            ))
    }
}
