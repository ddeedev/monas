pub mod popup;
pub mod search_bar;

pub use search_bar::SearchBar;

use gpui::{App, SharedString, Window};
use std::rc::Rc;

pub type OnSearch = Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>;
