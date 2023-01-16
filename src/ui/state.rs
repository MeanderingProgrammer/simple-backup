use crate::api;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        main {
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
