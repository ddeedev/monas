use crate::menu::init_app_menu;
use crate::view::main_content::MainContent;
use action::Quit;
use assets::{Assets, load_fonts_asset};
use component::theme;
use gpui::{
    App, Application, TitlebarOptions, WindowBackgroundAppearance, WindowKind, WindowOptions,
    point, prelude::*, px,
};
use ui::search::popup;

fn build_application() -> Application {
    let platform = gpui_platform::current_platform(false);
    Application::new_inaccessible(platform).with_http_client(std::sync::Arc::new(
        reqwest_client::ReqwestClient::user_agent("monas/0.1").unwrap(),
    ))
}

pub struct AppRunner;

impl AppRunner {
    pub fn run() {
        build_application().with_assets(Assets).run(|cx: &mut App| {
            // load asset
            load_fonts_asset(cx);
            theme::init(cx);
            action::register_keybind(cx);
            input::TextInput::bind_keys(cx);
            popup::Popup::bind_keys(cx);
            init_app_menu(cx);

            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.on_window_closed(|cx, _window| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            let titlebar = TitlebarOptions {
                traffic_light_position: Some(point(px(12.), px(12.))),
                appears_transparent: true,
                title: None,
            };
            let window_option = WindowOptions {
                titlebar: Some(titlebar),
                kind: WindowKind::Normal,
                is_movable: true,
                window_background: WindowBackgroundAppearance::Opaque,
                ..Default::default()
            };

            cx.open_window(window_option, |_window, cx| cx.new(MainContent::new))
                .expect("Fail to open main window");
            cx.activate(true);
        });
    }
}
