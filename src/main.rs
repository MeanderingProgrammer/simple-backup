use simple_backup::ui;

use dioxus::prelude::*;
use dioxus_router;
use dioxus_router::{
    Link,
    Route,
    Router,
};

struct Page<'a> {
    name: &'a str,
    route: &'a str,
    child: Element<'a>,
}

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let pages = vec![
        Page { name: "Home", route: "/", child: ui::profile::app(cx) },
        Page { name: "State", route: "/state", child: ui::state::app(cx) },
    ];

    cx.render(rsx!(
        // Make bulma.io style available on all pages
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
        },
        style { include_str!("ui/style.css") },
        Router {
            // Navigation bar for router
            nav {
                class: "navbar",
                pages.iter().map(|page| rsx!(
                    Link { class: "navbar-item", active_class: "is-active", to: page.route, page.name }
                )),
            },

            pages.iter().map(|page| rsx!(
                Route { to: page.route, &page.child }
            )),
        },
    ))
}
