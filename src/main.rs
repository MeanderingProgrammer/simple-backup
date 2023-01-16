use simple_backup::ui;

use dioxus::prelude::*;
use dioxus_desktop::{
    Config,
    WindowBuilder,
};
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

fn main() {
    let window = WindowBuilder::new()
        .with_title("Simple Backup Tool")
        .with_window_icon(None);

    dioxus_desktop::launch_cfg(
        app,
        Config::new().with_window(window),
    );
}

fn app(cx: Scope) -> Element {
    let pages = vec![
        Page { name: "Home", route: "/", child: |cx| ui::profile::app(cx) },
        Page { name: "State", route: "/state", child: |cx| ui::state::app(cx) },
    ];

    cx.render(rsx!(
        // Make bulma.io style available on all pages
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
        },
        // Custom style
        style { include_str!("ui/style.css") },
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
