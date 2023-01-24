use crate::api;
use crate::db::backup::BackupConfig;

use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        main {
            api::profile::get().iter().map(|directory| rsx!(
                div {
                    class: "box content",
                    p { strong { "Directory: " } "{directory.path}" }
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
                }
            )),
        },
    ))
}
