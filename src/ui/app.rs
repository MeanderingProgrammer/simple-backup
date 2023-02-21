use crate::ui;

use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};

pub fn entry_point(cx: Scope) -> Element {
    cx.render(rsx!(
        // Make bulma.io style available on all pages
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
        },
        // Custom style
        style { include_str!("style.css") },
        Router {
            // Navigation bar for router
            nav {
                class: "navbar",
                Link { class: "navbar-item", active_class: "is-active", to: "/", "Home" }
                Link { class: "navbar-item", active_class: "is-active", to: "/add", "Add" }
                Link { class: "navbar-item", active_class: "is-active", to: "/state", "State" }
                Link { class: "navbar-item", active_class: "is-active", to: "/helpers", "Helpers" }
            },
            // The actual route definitions
            Route { to: "/", ui::profile::app {} }
            Route { to: "/add", ui::add::app {} }
            Route { to: "/state", ui::state::app {} }
            Route { to: "/helpers", ui::helpers::app {} }
        },
    ))
}
