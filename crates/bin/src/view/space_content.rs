use context::space::SpaceContext;
use gpui::Entity;
use ui::{search::popup::Popup, sidebar::SidebarView};

#[derive(Debug, Clone)]
pub struct SpaceContent {
    pub space: Entity<SpaceContext>,
    pub sidebar: Entity<SidebarView>,
    pub popup_search: Entity<Popup>,
}

impl SpaceContent {
    pub fn new(
        space: Entity<SpaceContext>,
        sidebar: Entity<SidebarView>,
        popup_search: Entity<Popup>,
    ) -> Self {
        Self {
            space,
            sidebar,
            popup_search,
        }
    }
}
