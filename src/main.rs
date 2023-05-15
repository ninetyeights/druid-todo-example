use druid::{AppLauncher, WindowDesc};

mod controllers;
mod delegate;
use delegate::Delegate;

mod data;

use data::AppState;

mod view;

use view::build_ui;
use crate::data::TodoItem;

pub fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Todo Tutorial")
        .window_size((400.0, 400.0))
        .show_titlebar(true);

    let initial_state = AppState::load_from_json();

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application");
}