use crate::api;

use dioxus::prelude::*;
use itertools::Itertools;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(main {
        rsx!(button {
            class: "button",
            onclick: |_| {
                api::state::sync();
                // Trigger reload in case of change
                cx.needs_update();
            },
            "Sync"
        })
        api::state::previous().iter()
            .sorted()
            .group_by(|state| state.root.clone())
            .into_iter()
            .map(|(root, group)| rsx!(div {
                class: "box content",
                p { strong { "Local Directory: " } "{root}" }
                table {
                    class: "table",
                    thead {
                        tr {
                            th { "File Path" }
                            th { "Last Updated" }
                        }
                    }
                    tbody {
                        group.map(|state| rsx!(
                            tr {
                                td { "{state.suffix}" }
                                td { "{state.to_date(DATE_FORMAT)}" }
                            }
                        ))
                    }
                }
            })),
    }))
}
