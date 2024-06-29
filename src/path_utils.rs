use std::env;
const PATH_KEY: &str = "FAKE_PATH"; // TODO: Change This When Ready

pub fn get_raw_path() -> String {
    env::var(PATH_KEY).unwrap_or_else(|_| {
        // TODO:: log this
        env::set_var(PATH_KEY, "");
        return "".to_string()
    })
}

pub fn write_raw_path(p: &str) {
    env::set_var(PATH_KEY, p)
}
