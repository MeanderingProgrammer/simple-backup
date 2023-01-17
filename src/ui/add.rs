use crate::db::profile::{
    AwsS3Config,
    BackupConfig,
    LocalConfig,
};

use dioxus::prelude::*;
use native_dialog::FileDialog;

#[inline_props]
#[allow(non_snake_case)]
fn SelectFolder<'a>(cx: Scope<'a>, directory_type: String, on_select: EventHandler<'a, String>) -> Element {
    let folder = use_state(&cx, String::default);
    cx.render(rsx!(
        div {
            class: "file has-name is-fullwidth",
            span {
                class: "file-cta",
                onclick: move |_| {
                    let directory = select_directory().unwrap_or_default();
                    folder.set(directory.clone());
                    on_select.call(directory.clone());
                },
                span { class: "file-icon", i { class: "fas fa-upload" } }
                span { class: "file-label", "Choose {directory_type} directory…" }
            }
            span { class: "file-name", "{folder}" }
        }
    ))
}

#[inline_props]
#[allow(non_snake_case)]
fn SimpleInput<'a>(cx: Scope<'a>, helper_text: String, on_input: EventHandler<'a, String>) -> Element {
    let property = use_state(&cx, String::default);
    cx.render(rsx!(
        input {
            class: "input is-primary is-fullwidth",
            placeholder: "{helper_text}",
            value: "{property}",
            oninput: move |event| {
                let value = event.value.to_string();
                property.set(value.clone());
                on_input.call(value.clone());
            },
        }
    ))
}

#[inline_props]
#[allow(non_snake_case)]
fn LocalConfig(cx: Scope, backup_config: UseState<BackupConfig>) -> Element {
    cx.render(rsx!(
        SelectFolder {
            directory_type: "backup".to_string(),
            on_select: move |path| backup_config.set(BackupConfig::Local(LocalConfig {
                path: path
            })),
        }
    ))
}

#[inline_props]
#[allow(non_snake_case)]
fn AwsS3Config(cx: Scope, backup_config: UseState<BackupConfig>) -> Element {
    let bucket = use_state(&cx, String::default);
    let key = use_state(&cx, String::default);

    let new_config = BackupConfig::AwsS3(AwsS3Config {
        bucket: bucket.to_string(),
        key: key.to_string(),
    });
    if &new_config != backup_config.get() {
        backup_config.set(new_config);
    }

    cx.render(rsx!(
        SimpleInput {
            helper_text: "S3 Bucket".to_string(),
            on_input: move |input| bucket.set(input),
        }
        SimpleInput {
            helper_text: "S3 Key".to_string(),
            on_input: move |input| key.set(input),
        }
    ))
}

pub fn app(cx: Scope) -> Element {
    let local_path = use_state(&cx, String::default);
    let backup_config = use_state(&cx, BackupConfig::default);
    cx.render(rsx!(
        main {
            h4 { class: "title is-4", "Tracking Settings" }
            SelectFolder {
                directory_type: "input".to_string(),
                on_select: |path| local_path.set(path),
            }

            h4 { class: "title is-4", "Backup Settings" }
            div {
                class: "select is-primary is-fullwidth",
                select {
                    onchange: move |event| {
                        let updated_backup_config = match event.value.as_str() {
                            "Local" => BackupConfig::Local(LocalConfig::default()),
                            "AWS" => BackupConfig::AwsS3(AwsS3Config::default()),
                            _ => panic!("Unhandled backup option")
                        };
                        backup_config.set(updated_backup_config);
                    },
                    option { "Local" }
                    option { "AWS" }
                }
            }
            match backup_config.get() {
                BackupConfig::Local(_) => rsx!(
                    LocalConfig {
                        backup_config: backup_config.clone(),
                    }
                ),
                BackupConfig::AwsS3(_) => rsx!(
                    AwsS3Config {
                        backup_config: backup_config.clone(),
                    }
                ),
            }

            button {
                class: "button is-primary is-fullwidth",
                onclick: move |_| submit(local_path, backup_config),
                "Submit"
            }
        }
    ))
}

fn select_directory() -> Option<String> {
    let selected_path = FileDialog::new()
        .show_open_single_dir()
        .unwrap();
    match selected_path {
        Some(path) => {
            let directory = path.to_str().unwrap();
            Some(directory.to_string())
        },
        None => None,
    }
}

fn submit(local_path: &UseState<String>, backup_config: &UseState<BackupConfig>) {
    println!("{:?}", local_path);
    println!("{:?}", backup_config);
}
