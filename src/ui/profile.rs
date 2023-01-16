use crate::api;

use dioxus::prelude::*;
use native_dialog::FileDialog;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        main {
            api::profile::get().iter().map(|directory| {
                rsx!(
                    div {
                        class: "title",
                        "{directory}",
                    }
                )
            }),
            rsx!(
                button {
                    class: "button",
                    onclick: |_| add_directory(),
                    "Add",
                },
                button {
                    class: "button",
                    onclick: |_| sync_state(),
                    "Sync",
                },
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

fn sync_state() {
    let previous_state = api::state::previous();
    dbg!(&previous_state);

    let current_state = api::state::current();
    dbg!(&current_state);

    dbg!(previous_state == current_state);

    previous_state.difference(&current_state)
        .for_each(|diff| {
            dbg!(diff);
        });
}
