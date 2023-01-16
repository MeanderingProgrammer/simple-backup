use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
        },
        main {
            div {
                "TODO!"
            }
        },
    ))
}
