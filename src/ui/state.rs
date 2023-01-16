use crate::api;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    println!("3");

    cx.render(rsx!(
        main {
            rsx!(
                button { class: "button", onclick: |_| sync_state(cx), "Sync" },
            ),
            api::state::previous().iter().map(|file| rsx!(
                div {
                    class: "box",
                    p { strong { "File Path: " } "{file.path}" }
                    p { strong { "Last Updated: " } "{format_date(file.to_date())}" }
                }
            )),
        },
    ))
}

fn format_date(date_time: DateTime<Utc>) -> String {
    date_time.format("%Y-%m-%d %H:%M").to_string()
}

fn sync_state(cx: Scope) {
    let previous_state = api::state::previous();
    let current_state = api::state::current();

    dbg!(previous_state == current_state);

    let difference = previous_state.difference(&current_state);
    dbg!(difference);

    // Trigger reload on change
    cx.needs_update();
}
