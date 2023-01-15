mod user_profile;
mod system_state;

use self::user_profile::UserProfile;
use self::system_state::{
    FileState,
    SystemState,
};

use dioxus::prelude::*;
use glob::glob;
use native_dialog::FileDialog;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let profile = get_profile();

    cx.render(rsx!(
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
        },
        main {
            profile.iter().map(|directory| {
                rsx!(
                    div {
                        class: "title",
                        "{directory}",
                    }
                )
            }),
            rsx!(
                button {
                    class: "button",
                    onclick: |_| select_file(),
                    "Add",
                }
            ),
        },
    ))
}

fn select_file() {
    let path = FileDialog::new()
        .show_open_single_dir()
        .unwrap().unwrap();
    dbg!(path);
}

fn run_process() {
    let profile = get_profile();

    let previous_state = SystemState::read();
    dbg!(&previous_state);

    let current_state = get_system_state(&profile);
    dbg!(&current_state);

    dbg!(previous_state == current_state);

    previous_state.difference(&current_state)
        .for_each(|diff| {
            dbg!(diff);
        });
}

fn get_profile() -> UserProfile {
    let mut profile = UserProfile::read();
    //profile.add("C:/Users/vsusl/Documents/other/important");
    //profile.add("target/debug/deps/");
    //profile.save();
    //dbg!(&profile);
    profile
}

fn get_system_state(profile: &UserProfile) -> SystemState {
    let mut system_state = SystemState::new();
    profile.iter()
        .for_each(|directory| add_file_states(&mut system_state, &directory));
    //system_state.save();
    system_state
}

fn add_file_states(system_state: &mut SystemState, root: &str) {
    let glob_pattern = format!("{}/**/*", root);
    glob(&glob_pattern).unwrap()
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .for_each(|path| system_state.add(FileState::new(path)));
}
