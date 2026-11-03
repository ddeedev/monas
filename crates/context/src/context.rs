pub mod api;
pub mod browser;
pub mod config;
pub mod node;
pub mod tabs;

pub use browser::space;
pub use browser::tab::BrowserData;
pub use tabs::{Tab, TabData};
