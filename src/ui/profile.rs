use crate::api;

use dioxus::prelude::*;
use native_dialog::FileDialog;

pub fn app(cx: Scope) -> Element {
    println!("1");

    cx.render(rsx!(
        main {
            api::profile::get().iter().map(|directory| rsx!(
                div {
                    class: "box",
                    p { strong { "Directory: " } "{directory}" }
                }
            )),
            rsx!(
                button { class: "button", onclick: |_| add_directory(cx), "Add" },
            ),
        },
    ))
}

fn add_directory(cx: Scope) {
    let selected_path = FileDialog::new()
        .show_open_single_dir()
        .unwrap();
    match selected_path {
        Some(path) => {
            let directory = path.to_str().unwrap();
            api::profile::add_directory(directory);
            // Trigger reload on change
            cx.needs_update();
        },
        // No path selected, nothing to do
        None => (),
    };
}
