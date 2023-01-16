use crate::api;

use dioxus::prelude::*;
use native_dialog::FileDialog;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        main {
            api::profile::get().iter().map(|directory| rsx!(
                div {
                    class: "box",
                    p { strong { "Directory: " } "{directory}" }
                }
            )),
            rsx!(
                button { class: "button", onclick: |_| add_directory(), "Add" },
            ),
        },
    ))
}

fn add_directory() {
    let path = FileDialog::new()
        .show_open_single_dir()
        .unwrap().unwrap();
    let directory = path.to_str().unwrap();
    api::profile::add_directory(directory);
}
