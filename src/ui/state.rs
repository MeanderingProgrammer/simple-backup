use crate::api;
use crate::db::profile::UserProfile;
use crate::db::state::FileState;

use dioxus::prelude::*;
use std::collections::HashSet;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        main {
            rsx!(
                button { class: "button", onclick: |_| sync_state(cx), "Sync" },
            ),
            api::state::previous().iter().map(|file| rsx!(
                div {
                    class: "box content",
                    p { strong { "File Path: " } "{file.path}" }
                    p { strong { "Last Updated: " } "{file.to_date(DATE_FORMAT)}" }
                }
            )),
        },
    ))
}

fn sync_state(cx: Scope) {
    let previous_state = api::state::previous();
    let current_state = api::state::current();

    dbg!(previous_state == current_state);

    let difference = previous_state.difference(&current_state);
    dbg!(&difference);

    let profile = api::profile::get();

    copy_files(&profile, &difference.added);
    copy_files(&profile, &difference.modified);

    current_state.save();

    // Trigger reload in case of change
    cx.needs_update();
}

fn copy_files(profile: &UserProfile, files: &HashSet<FileState>) {
    for file in files {
        let config = profile.get(&file.root);
        config.backup_config.copy_file(file);
    }
}
