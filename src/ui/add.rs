use crate::db::profile::{
    AwsS3Config,
    BackupConfig,
    LocalConfig,
};

use dioxus::prelude::*;
use native_dialog::FileDialog;
use std::borrow::Borrow;

#[derive(Props, PartialEq)]
struct SelectFolderProps {
    directory_type: String,
    path: UseState<String>,
}

#[allow(non_snake_case)]
fn SelectFolder(cx: Scope<SelectFolderProps>) -> Element {
    println!("2.1");

    cx.render(rsx!(
        div {
            class: "file has-name is-fullwidth",
            span {
                class: "file-cta",
                onclick: |_| {
                    let directory = select_directory().unwrap_or_default();
                    cx.props.path.set(directory);
                },
                span { class: "file-icon", i { class: "fas fa-upload" } }
                span { class: "file-label", "Choose {cx.props.directory_type} directory…" }
            }
            span { class: "file-name", "{cx.props.path}" }
        }
    ))
}

#[derive(Props, PartialEq)]
struct ConfigProps {
    backup_config: UseState<BackupConfig>,
}

#[allow(non_snake_case)]
fn LocalConfig(cx: Scope<ConfigProps>) -> Element {
    println!("2.2");

    let path = use_state(cx, String::default);
    println!("{:?}", path);

    //cx.props.backup_config.set(BackupConfig::Local(LocalConfig {
    //    path: path.to_string(),
    //}));

    cx.render(rsx!(
        SelectFolder {
            directory_type: "backup".to_string(),
            path: path.clone(),
        }
    ))
}

#[allow(non_snake_case)]
fn AwsS3Config(cx: Scope<ConfigProps>) -> Element {
    println!("2.3");

    let bucket = use_state(cx, String::default);
    let key = use_state(cx, String::default);
    println!("{:?} -- {:?}", bucket, key);

    //cx.props.backup_config.set(BackupConfig::Local(LocalConfig {
    //    path: path.to_string(),
    //}));

    cx.render(rsx!(
        input {
            class: "input is-primary is-fullwidth",
            placeholder: "S3 Bucket",
            oninput: |event| {
                bucket.set(event.value.to_string());
                cx.props.backup_config.set(BackupConfig::AwsS3(AwsS3Config {
                    bucket: bucket.to_string(),
                    key: key.to_string(),
                }));
            },
        }
        input {
            class: "input is-primary is-fullwidth",
            placeholder: "S3 Key",
            oninput: |event| {
                key.set(event.value.to_string());
                cx.props.backup_config.set(BackupConfig::AwsS3(AwsS3Config {
                    bucket: bucket.to_string(),
                    key: key.to_string(),
                }));
            },
        }
    ))
}

pub fn app(cx: Scope) -> Element {
    println!("2");

    let local_path = use_state(cx, String::default);
    println!("{:?}", local_path);

    let backup_config = use_state(cx, BackupConfig::default);
    println!("{:?}", backup_config);

    cx.render(rsx!(
        main {
            p { strong { "Fill in Details" } }

            SelectFolder {
                directory_type: "input".to_string(),
                path: local_path.clone(),
            }

            div {
                class: "select is-primary is-fullwidth",
                select {
                    onchange: |event| {
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

            match backup_config.current().borrow() {
                BackupConfig::Local(_) => rsx! (
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
                onclick: |_| submit(local_path, backup_config),
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
