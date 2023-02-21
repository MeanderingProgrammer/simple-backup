use native_dialog::{FileDialog, MessageDialog, MessageType};

pub fn info(title: &str, text: &str) -> bool {
    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title(title)
        .set_text(text)
        .show_confirm()
        .unwrap()
}

pub fn error(title: &str, text: &str) {
    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title(title)
        .set_text(text)
        .show_alert()
        .unwrap();
}

pub fn directory_select() -> Option<String> {
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
