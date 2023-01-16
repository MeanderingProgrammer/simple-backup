use crate::ui;

use dioxus::prelude::*;
use dioxus_router::{
    Link,
    Route,
    Router,
};

struct Page<'a> {
    name: &'a str,
    route: &'a str,
    child: fn(Scope) -> Element,
}

pub fn entry_point(cx: Scope) -> Element {
    let pages = vec![
        Page { name: "Home", route: "/", child: |cx| ui::profile::app(cx) },
        Page { name: "Add", route: "/add", child: |cx| ui::add::app(cx) },
        Page { name: "State", route: "/state", child: |cx| ui::state::app(cx) },
    ];

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
                pages.iter().map(|page| rsx!(
                    Link { class: "navbar-item", active_class: "is-active", to: page.route, page.name }
                )),
            },

            // The actual route definitions
            pages.iter().map(|page| rsx!(
                Route { to: page.route, (page.child)(cx) }
            )),
        },
    ))
}
