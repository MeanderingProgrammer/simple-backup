use crate::api::profile;
use crate::backup::backup::BackupConfig;
use crate::backup::local::LocalConfig;

use dioxus::prelude::*;

const BACKUP_PREFIX: &str = "C:\\Users\\vsusl\\Documents\\scripts\\backup-test\\file-backup\\";

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(main {
        button {
            class: "button is-dark is-fullwidth",
            onclick: move |_| {
                add_directory("file-local", "file-backup");
                add_directory("local", "backup");
            },
            "Add Default Profile"
        }
    }))
}

fn add_directory(input_directory: &str, output_directory: &str) {
    profile::add_directory(
        format!("{}{}", BACKUP_PREFIX, input_directory),
        BackupConfig::Local(LocalConfig {
            path: format!("{}{}", BACKUP_PREFIX, output_directory),
        }),
    );
}
