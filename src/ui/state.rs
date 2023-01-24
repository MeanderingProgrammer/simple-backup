use crate::api;

use dioxus::prelude::*;
use itertools::Itertools;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        main {
            rsx!(
                button { class: "button", onclick: |_| sync_state(cx), "Sync" },
            ),
            api::state::previous().iter()
                .sorted()
                .map(|file| rsx!(
                    div {
                        class: "box content",
                        p { strong { "File Path: " } "{file.path}" }
                        p { strong { "Last Updated: " } "{file.to_date(DATE_FORMAT)}" }
                    }
                )),
        },
    ))
}

fn sync_state(cx: Scope) {
    api::state::sync();
    // Trigger reload in case of change
    cx.needs_update();
}
