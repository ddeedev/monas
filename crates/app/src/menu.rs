use action::{Quit, ToggleSidebar};
use gpui::{App, Menu, MenuItem, SystemMenuType};

pub fn init_app_menu(cx: &mut App) {
    cx.set_menus(vec![
        Menu {
            name: "Silly".into(),
            disabled: false,
            items: vec![
                MenuItem::os_submenu("Services", SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        },
        Menu {
            disabled: false,
            name: "File".into(),
            items: vec![MenuItem::separator()],
        },
        Menu {
            name: "View".into(),
            disabled: false,
            items: vec![
                MenuItem::separator(),
                MenuItem::action("Toggle Sidebar", ToggleSidebar),
            ],
        },
        Menu {
            name: "Help".into(),
            disabled: false,
            items: vec![MenuItem::separator()],
        },
    ])
}
