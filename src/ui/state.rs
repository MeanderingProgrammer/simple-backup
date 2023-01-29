use crate::api;
use crate::ui;

use dioxus::prelude::*;
use itertools::Itertools;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(main {
        rsx!(button {
            class: "button is-primary is-fullwidth",
            onclick: |_| {
                api::state::sync();
                // Trigger reload in case of change
                cx.needs_update();
            },
            "Sync"
        })
        api::state::previous().iter()
            .sorted()
            .group_by(|state| state.owner_id.clone())
            .into_iter()
            .map(|(profile_id, group)| {
                let directory = api::profile::get_by_id(&profile_id);
                rsx!(div {
                    class: "box content",
                    ui::profile::render_directory {
                        directory: directory.clone(),
                        allow_delete: false,
                    }
                    table {
                        class: "table",
                        thead { tr {
                            th { "File Path" }
                            th { "Last Updated" }
                        }}
                        tbody {
                            group.map(|state| rsx!(tr {
                                td { "{state.suffix}" }
                                td { "{state.to_date(DATE_FORMAT)}" }
                            }))
                        }
                    }
                })
            })
    }))
}
