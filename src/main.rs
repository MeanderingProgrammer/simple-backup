mod user_profile;
mod system_state;

use self::user_profile::UserProfile;
use self::system_state::{
    FileState,
    SystemState,
};

use glob::glob;

fn main() {
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
