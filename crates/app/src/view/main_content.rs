use std::vec;

use super::mock;
use crate::view::space_content::SpaceContent;
use action::{OpenSearchPopup, ResetSidebar, SwitchSpace, ToggleSidebar};
use context::space::{SidebarContext, SpaceContext};
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Global, SharedString, Window, div, prelude::*, px,
};
use input::TextInput;
use ui::{
    ActiveTheme,
    search::popup::{Popup, PopupEvent},
    sidebar::SidebarView,
};

impl Global for MainContent {}

pub struct MainContent {
    pub current_space: usize,
    pub spaces: Vec<SpaceContent>,
    pub space_logos: Vec<String>,
    pub focus_handle: FocusHandle,
}

impl MainContent {
    //pub fn global(cx: &App) -> &Self {
    //    cx.global()
    //}

    // TODO::
    // Using db or local space file instead of hardcode
    pub fn new(cx: &mut Context<Self>) -> Self {
        let current_space = 1;

        let contexts = mock::spaces();
        let logos: Vec<String> = contexts
            .iter()
            .filter_map(|s| s.name.chars().next())
            .map(|c| c.to_string())
            .collect();

        let spaces: Vec<SpaceContent> = contexts
            .into_iter()
            .map(|ctx| Self::build_space(cx, ctx))
            .collect();

        for space in &spaces {
            space.sidebar.update(cx, |sidebar, sb_cx| {
                sidebar.register_logos(sb_cx, logos.clone());
                sidebar.set_active_space(sb_cx, current_space);
            });
        }

        Self {
            current_space,
            spaces,
            focus_handle: cx.focus_handle(),
            space_logos: logos,
        }
    }

    fn build_space(cx: &mut Context<Self>, mut space_ctx: SpaceContext) -> SpaceContent {
        let name: SharedString = space_ctx.name.clone().into();
        let number = space_ctx.number;

        let sidebar_ctx = std::mem::replace(&mut space_ctx.sidebar, SidebarContext::new());
        let sidebar_entity = cx.new(|_| sidebar_ctx);
        cx.observe(&sidebar_entity, |_, _, cx| cx.notify()).detach();

        let search_input = cx.new(|cx| TextInput::new("Search or Enter URL...", cx));
        let sidebar = cx.new(|_| SidebarView::new(name, number, sidebar_entity, search_input));
        //cx.subscribe(&sidebar, |_, dock, event: &SidebarEvent, cx| {
        //    match event {
        //        SidebarEvent::Toggle => {
        //            dock.update(cx, |dock, cx| dock.toggle(cx));
        //        }
        //        SidebarEvent::Search(_data) => {}
        //    }
        //    cx.notify();
        //})
        //.detach();

        let space = cx.new(|_| space_ctx);
        cx.observe(&space, |_, _, cx| cx.notify()).detach();

        let popup_text = cx.new(|cx| TextInput::new("", cx));
        let popup = cx.new(|cx| Popup::new(popup_text, vec![], false, cx));

        cx.subscribe(&popup, |_, popup, event: &PopupEvent, cx| {
            match event {
                PopupEvent::Dismiss => {
                    popup.update(cx, |p, cx| p.close(cx));
                }
                PopupEvent::Open(_data) => {}
                PopupEvent::Search(_text) => {
                    // TODO: run a search / navigate with `_text`
                }
            }
            cx.notify();
        })
        .detach();

        SpaceContent::new(space, sidebar, popup)
    }

    fn sync_sidebars(&self, cx: &mut Context<Self>) {
        let logos = self.retrive_logos();
        let active = self.current_space;
        for space in &self.spaces {
            space.sidebar.update(cx, |sidebar, sb_cx| {
                sidebar.register_logos(sb_cx, logos.clone());
                sidebar.set_active_space(sb_cx, active);
            });
        }
    }

    pub fn _new_space(&mut self, cx: &mut Context<Self>, space_ctn: SpaceContent) {
        if let Some(logo) = space_ctn.space.read(cx).name.chars().nth(0) {
            self.space_logos.push(logo.to_string());
        }
        self.spaces.push(space_ctn);
        self.current_space = self.spaces.len();
        self.sync_sidebars(cx);
        cx.notify();
    }

    pub fn _with_space(&mut self, cx: &mut Context<Self>, space_ctns: Vec<SpaceContent>) {
        for space_ctn in space_ctns.into_iter() {
            if let Some(logo) = space_ctn.space.read(cx).name.chars().nth(0) {
                self.space_logos.push(logo.to_string());
            }
            self.spaces.push(space_ctn);
        }
        self.current_space = 1;
        self.sync_sidebars(cx);
        cx.notify();
    }

    pub fn switch_space(&mut self, cx: &mut Context<Self>, number: usize) {
        if number > 0 && number <= self.spaces.len() {
            self.current_space = number;
            self.sync_sidebars(cx);
            cx.notify();
        }
    }

    pub fn current_sidebar(&self) -> Entity<SidebarView> {
        self.spaces[self.current_space - 1].sidebar.clone()
    }

    pub fn current_search(&self) -> Entity<Popup> {
        self.spaces[self.current_space - 1].popup_search.clone()
    }

    pub fn retrive_logos(&self) -> Vec<String> {
        self.space_logos.clone()
    }
}

impl Focusable for MainContent {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainContent {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let handle = self.focus_handle.clone();
        if !handle.contains_focused(window, cx) {
            window.focus(&handle, cx);
        }
        window.on_next_frame(move |window, cx| {
            if !handle.contains_focused(window, cx) {
                window.focus(&handle, cx);
            }
        });

        // sidebar
        let current_sidebar = self.current_sidebar();
        let sidebar_fraction = current_sidebar.read(cx).visible_fraction();

        // popup
        let current_popup = self.current_search();

        let theme = cx.theme().clone();

        div()
            .key_context("main_view")
            .track_focus(&self.focus_handle)
            .font_family(theme.font_family.clone())
            .text_size(theme.font_size)
            .text_color(theme.theme_color.text)
            .bg(theme.theme_color.background)
            .on_action(cx.listener(|this, _: &ToggleSidebar, _window, cx| {
                this.current_sidebar()
                    .update(cx, |sidebar, sb_cx| sidebar.toggle(sb_cx));
            }))
            .on_action(cx.listener(|this, action: &SwitchSpace, _window, cx| {
                this.switch_space(cx, action.0);
            }))
            .on_action(cx.listener(|this, _: &ResetSidebar, _window, cx| {
                this.current_sidebar()
                    .update(cx, |sidebar, sb_cx| sidebar.reset(sb_cx));
            }))
            .on_action(cx.listener(|this, _: &OpenSearchPopup, window, cx| {
                this.current_search()
                    .update(cx, |popup, cx| popup.open(window, cx));
            }))
            .flex()
            .flex_row()
            .size_full()
            .child(current_sidebar)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .m_2()
                    .ml(px(8.0 * (1.0 - sidebar_fraction)))
                    // 2. The styling for the card stroke containe
                    .bg(theme.theme_color.tile_bg)
                    //.bg(theme.theme_color.tile_bg)
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(theme.theme_color.border)
                    // Layout and inner children styling
                    .p_4()
                    .gap_3()
                    .justify_center()
                    .items_center()
                    .text_xl()
                    .text_color(theme.theme_color.text)
                    .on_mouse_move(cx.listener(|this, _: &gpui::MouseMoveEvent, _window, cx| {
                        this.current_sidebar()
                            .update(cx, |sidebar, cx| sidebar.set_floating_visible(false, cx));
                    })),
            )
            // Web view UI
            .child(div().absolute().size_full().child(current_popup))
    }
}
