use simple_backup::ui::app;

use dioxus_desktop::{Config, WindowBuilder, tao::window::Icon};

fn main() {
    let window = WindowBuilder::new()
        .with_title("Simple Backup Tool")
        .with_window_icon(Some(get_icon()));

    dioxus_desktop::launch_cfg(
        app::entry_point,
        Config::new().with_window(window),
    );
}

fn get_icon() -> Icon {
    let icon_bytes = include_bytes!("../assets/icon.ico").to_vec();

    let total_pixels = icon_bytes.len() / 4;
    let side_length_float = (total_pixels as f64).sqrt();
    let side_length = side_length_float as u32;

    Icon::from_rgba(icon_bytes, side_length, side_length).unwrap()
}
