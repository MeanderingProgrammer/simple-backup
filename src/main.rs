use simple_backup::api;

use dioxus::prelude::*;
use native_dialog::FileDialog;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
        },
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
                    onclick: |_| select_file(),
                    "Add",
                }
            ),
        },
    ))
}

fn select_file() {
    let path = FileDialog::new()
        .show_open_single_dir()
        .unwrap().unwrap();
    let directory = path.to_str().unwrap();
    api::profile::add_directory(directory);
}

fn run_process() {
    let profile = api::profile::get();

    let previous_state = api::state::previous();
    dbg!(&previous_state);

    let current_state = api::state::current(&profile);
    dbg!(&current_state);

    dbg!(previous_state == current_state);

    previous_state.difference(&current_state)
        .for_each(|diff| {
            dbg!(diff);
        });
}
