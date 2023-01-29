use crate::api;
use crate::backup::backup::BackupConfig;
use crate::db::profile::DirectoryConfig;

use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(main {
        api::profile::get().iter()
            .map(|directory| rsx!(div {
                class: "box content",
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
