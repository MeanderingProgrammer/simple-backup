use crate::api;

use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    println!("1");

    cx.render(rsx!(
        main {
            api::profile::get().iter().map(|directory| rsx!(
                div {
                    class: "box",
                    p { strong { "Directory: " } "{directory.local_path}" }
                }
            )),
        },
    ))
}
