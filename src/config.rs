#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub lines: bool,
    pub recursive: bool,
    pub full_recursive: bool,
    pub count: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            lines: false,
            recursive: false,
            full_recursive: false,
            count: false,
        }
    }
}
