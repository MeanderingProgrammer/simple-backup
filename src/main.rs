use simple_backup::ui;

use dioxus::prelude::*;
use dioxus_router::{
    Route,
    Router,
};

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        Router {
            Route { to: "/", ui::profile::app(cx) },
            Route { to: "/state", ui::state::app(cx) },
        },
    ))
}
