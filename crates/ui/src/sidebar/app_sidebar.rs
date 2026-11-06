use crate::Theme;
use crate::search::{OnSearch, SearchBar};
use crate::sidebar::ToggleHadler;
use component::list_row::ListRow;
use component::tile::Tile;
use component::window_control::WindowControl;
use component::{button::Button, favicon::Favicon};

use action::SwitchSpace;
use context::{
    TabData,
    node::{NodeData, NodeId},
    space::SidebarContext,
};
use gpui::{
    AnyElement, App, Entity, IntoElement, Rgba, SharedString, Window, div, prelude::*, px, svg,
    white,
};

use crate::ActiveTheme;
use input::TextInput;

#[derive(IntoElement)]
pub struct AppSidebar {
    width: f32,
    on_toggle: Option<ToggleHadler>,
    space_name: SharedString,
    active_space: usize,
    entity: Entity<SidebarContext>,
    space_logos: Vec<String>,
    search_input: Entity<TextInput>,
    on_search: Option<OnSearch>,
}

impl AppSidebar {
    pub fn new(
        space_name: SharedString,
        active_space: usize,
        entity: Entity<SidebarContext>,
        space_logos: Vec<String>,
        search_input: Entity<TextInput>,
    ) -> Self {
        Self {
            width: 240.0,
            on_toggle: None,
            space_name,
            active_space,
            space_logos,
            entity,
            search_input,
            on_search: None,
        }
    }

    pub fn on_search(mut self, f: OnSearch) -> Self {
        self.on_search = Some(f);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn toggle_sidebar(mut self, f: ToggleHadler) -> Self {
        self.on_toggle = Some(f);
        self
    }
}

impl AppSidebar {
    fn render_header_with_toggle(
        &self,
        on_toggle: Option<ToggleHadler>,
        t: &Theme,
    ) -> impl IntoElement {
        let (hover, outline) = (t.theme_color.tile_bg, t.theme_color.sidebar_outline);

        let toggle_btn = Button::new("sidebar-toggle")
            .w(px(30.0))
            .h(px(28.0))
            .focusable(false)
            .rounded(px(6.0))
            .cursor_pointer()
            .opacity(0.8)
            .icon("icons/sidebar-left.svg")
            .icon_h(px(16.))
            .icon_w(px(18.))
            .icon_color(outline)
            .hover(move |style| style.bg(hover))
            .when_some(on_toggle, move |btn, f| {
                btn.on_click(move |event, window, cx| f(event, window, cx))
            });

        div()
            .mt_2()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .pb_2()
            // NOTE: 1.2 Enable custom traffic light
            .child(WindowControl::new("main-traffic-lights").idle(t.theme_color.background))
            .child(toggle_btn)
    }

    fn render_workspace_card(
        &self,
        workspace_name: SharedString,
        icon: Option<SharedString>,
        t: &Theme,
    ) -> impl IntoElement {
        let (text, bg, tile, space_name_font) = (
            t.theme_color.text,
            t.theme_color.background,
            t.theme_color.tile_bg,
            t.space_name_font.clone(),
        );
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap_2()
            .bg(bg)
            .rounded(px(6.0))
            .border_color(tile)
            .text_xl()
            .text_center()
            .text_color(text)
            .opacity(0.5)
            .font_family(space_name_font)
            .when_some(icon, |this, emo| this.child(div().child(emo)))
            //.child(div().child("⭐"))
            .child(
                div()
                    .child(workspace_name)
                    .font_weight(gpui::FontWeight::EXTRA_BOLD),
            )
    }

    // TODO::  fix to more dynamic, 3*3  is perfet but maybe we can make it dynamic or limi r * c
    fn render_essential(&self, tabs: Vec<&TabData>) -> impl IntoElement {
        let width = self.width;
        let fraction = if width > 242. || width / 3.0 > 80.0 {
            0.31
        } else if width > 212. {
            0.48
        } else {
            1.0
        };

        div()
            .w_full()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap_2()
            .children(tabs.iter().enumerate().map(|(index, tab)| {
                let icon = match tab {
                    TabData::Browser(d) => d.favicon.clone(),
                };
                Tile::new(("essential", index))
                    .width(fraction)
                    .child(Favicon::new(icon).size(px(20.)))
            }))
    }

    fn render_space_selection(&self, t: &Theme) -> impl IntoElement {
        let (bg, tile, active) = (
            t.theme_color.background,
            t.theme_color.tile_bg,
            t.theme_color.tile_bg_active,
        );

        div()
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h(px(30.0))
            .text_color(white())
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_base()
            // LEFT: archive button
            .child(
                Button::new("space-selection-archive")
                    .label("A")
                    .cursor_pointer()
                    .on_click(move |_event, window, cx| {
                        window.dispatch_action(Box::new(SwitchSpace(1)), cx);
                    })
                    .rounded_full()
                    .w_8()
                    .h_full()
                    .bg(bg),
            )
            // CENTER: space logos
            .child(
                div()
                    .flex()
                    .h_full()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .gap_1()
                    .children(self.space_logos.iter().enumerate().map(|(idx, s)| {
                        let space_number = idx + 1;
                        let is_active = space_number == self.active_space;
                        Button::new(("space-selection", idx as u64))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_base()
                            .cursor_pointer()
                            .on_click(move |_event, window, cx| {
                                window.dispatch_action(Box::new(SwitchSpace(space_number)), cx);
                            })
                            .rounded(px(6.0))
                            .w_8()
                            .h_full()
                            .bg(if is_active { active } else { tile })
                            // TODO:: use space emoji
                            .label(s.to_string().to_uppercase())
                    })),
            )
            // RIGHT: add button
            .child(
                Button::new("space-selection-plus")
                    .label("+")
                    .text_base()
                    .rounded_full()
                    .w_8()
                    .h_full()
                    .bg(bg)
                    .on_click(move |_event, window, cx| {
                        window.dispatch_action(Box::new(SwitchSpace(1)), cx);
                    }),
            )
    }

    fn render_node(
        &self,
        space: &SidebarContext,
        id: &NodeId,
        depths: usize,
        t: &Theme,
    ) -> AnyElement {
        let Some(node) = space.nodes.get(id) else {
            return div().into_any_element();
        };
        let name = node.name.clone();

        match &node.data {
            NodeData::Tab { data, open } => self
                .render_tab(id, name.as_str(), data, *open, depths)
                .into_any_element(),
            NodeData::Folder { children, expand } => {
                let header = self.render_folder(id.clone(), name.as_str(), *expand, depths, t);
                div()
                    .flex()
                    .flex_col()
                    .child(header)
                    .when(*expand, |el| {
                        el.children(
                            children
                                .iter()
                                .map(|c| self.render_node(space, c, depths + 1, t)),
                        )
                    })
                    .into_any_element()
            }
        }
    }

    fn render_tab(
        &self,
        id: &NodeId,
        name: &str,
        data: &TabData,
        open: bool,
        depths: usize,
    ) -> AnyElement {
        ListRow::new(SharedString::from(id.to_string()), name.to_string())
            .depth(depths)
            .active(open)
            .leading(match data {
                TabData::Browser(b) => Favicon::new(b.favicon.clone()).size(px(18.)),
            })
            .into_any_element()
    }

    fn render_folder(
        &self,
        node_id: NodeId,
        name: &str,
        expand: bool,
        depths: usize,
        t: &Theme,
    ) -> impl IntoElement {
        let (fill, stroke) = (t.theme_color.folder_bg, t.theme_color.text);
        let entity = self.entity.clone();
        let click_id = node_id.clone();

        ListRow::new(SharedString::from(node_id.to_string()), name.to_string())
            .depth(depths)
            .leading(folder_icon(expand, fill, stroke))
            .on_click(move |_, _, cx| {
                entity.update(cx, |ctx, cx| {
                    ctx.toggle_folder(&click_id);
                    cx.notify();
                });
            })
    }
}

fn folder_icon(expand: bool, fill: Rgba, stroke: Rgba) -> impl IntoElement {
    let (fill_path, outline_path) = if expand {
        (
            "icons/folder-open-fill.svg",
            "icons/folder-open-outline.svg",
        )
    } else {
        ("icons/folder-fill.svg", "icons/folder-outline.svg")
    };
    div()
        .relative()
        .size(px(25.))
        .child(svg().path(fill_path).size(px(25.)).text_color(fill))
        .child(
            svg()
                .absolute()
                .top_0()
                .left_0()
                .path(outline_path)
                .size(px(25.))
                .text_color(stroke),
        )
}

impl RenderOnce for AppSidebar {
    fn render(mut self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme().clone();

        let mut search = SearchBar::new(self.search_input.clone());
        if let Some(f) = self.on_search.take() {
            search = search.on_search(f);
        }

        let on_toggle = self.on_toggle.take();
        let header = self.render_header_with_toggle(on_toggle, &theme);
        let space_name: SharedString = self.space_name.clone();
        let mut essential: Vec<&TabData> = Vec::new();
        let space = self.entity.read(cx);

        for item in &space.essential {
            let node = space.nodes.get(item);
            if let Some(node_data) = node
                && let NodeData::Tab {
                    data: item,
                    open: _,
                } = &node_data.data
            {
                essential.push(item);
            }
        }

        let folder = space.folder.clone();

        div()
            .w(px(self.width))
            .flex_shrink_0()
            .h_full()
            .bg(theme.theme_color.background)
            .font_family(theme.font_family.clone())
            .text_size(theme.font_size)
            .text_color(theme.theme_color.text)
            .flex()
            .flex_col()
            .child(header)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .m_2()
                    .mr_1()
                    .gap_2()
                    .child(search)
                    .child(self.render_essential(essential))
                    .child(self.render_workspace_card(space_name, None, &theme))
                    .child(
                        div()
                            .text_size(theme.folder_font_size)
                            .font_family(theme.folder_font.clone())
                            .gap_2()
                            .flex_1()
                            .children(folder.iter().map(|f| self.render_node(space, f, 0, &theme))),
                    )
                    .child(self.render_space_selection(&theme)),
            )
    }
}
