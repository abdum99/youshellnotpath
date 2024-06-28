use std::env;
const PATH_KEY: &str = "FAKE_PATH"; // TODO: Change This When Ready

pub fn get_raw_path() -> String {
    env::var(PATH_KEY).unwrap_or_else(|err| {
        panic!("could not fetch PATH {}", err)
    })
}
