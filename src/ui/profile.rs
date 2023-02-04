use crate::api;
use crate::backup::backup::BackupConfig;
use crate::db::profile::DirectoryConfig;

use dioxus::prelude::*;
use native_dialog::{
    MessageDialog,
    MessageType,
};

pub fn app(cx: Scope) -> Element {
    let profile = use_state(&cx, api::profile::get);
    let delete_mode = use_state(&cx, || false);
    cx.render(rsx!(main {
        rsx!(button {
            class: "button is-primary is-fullwidth",
            onclick: move |_| {
                delete_mode.set(!delete_mode.get());
            },
            if *delete_mode.get() { "Disable Delete" } else { "Enable Delete" }
        })
        profile.iter()
            .map(|directory| rsx!(div {
                class: "box content",

                if *delete_mode.get() {
                    rsx!(button {
                        class: "delete",
                        onclick: move |_| {
                            let response = MessageDialog::new()
                                .set_type(MessageType::Info)
                                .set_title("Are you sure you want to delete this backup?")
                                .set_text(&format!("{:#?}", directory.path))
                                .show_confirm()
                                .unwrap();

                            if response {
                                api::profile::delete_directory(&directory.id);
                                profile.set(api::profile::get());
                            }
                        },
                        "DELETE"
                    })
                }

                render_directory {
                    directory: directory.clone(),
                }
            }))
    }))
}

#[inline_props]
pub fn render_directory(cx: Scope, directory: DirectoryConfig) -> Element {
    cx.render(rsx!(
        p { strong { "Local Directory: " } "{directory.path}" }
        match &directory.backup_config {
            BackupConfig::Local(config) => rsx!(
                p { strong { "Type: " } "LOCAL" }
                p { strong { "Backup Location: " } "{config.path}" }
            ),
            BackupConfig::AwsS3(config) => rsx!(
                p { strong { "Type: " } "AWS S3" }
                p { strong { "Backup Bucket: " } "{config.bucket}" }
                p { strong { "Backup Key: " } "{config.key}" }
            ),
        }
    ))
}
