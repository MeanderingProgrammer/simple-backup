use simple_backup::ui::app;

use dioxus_desktop::{
    Config,
    WindowBuilder,
};

fn main() {
    let window = WindowBuilder::new()
        .with_title("Simple Backup Tool")
        .with_window_icon(None);

    dioxus_desktop::launch_cfg(
        app::entry_point,
        Config::new().with_window(window),
    );
}
