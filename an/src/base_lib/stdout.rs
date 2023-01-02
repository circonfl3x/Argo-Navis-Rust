use std::fmt::Display;
pub struct Stdout;

impl Stdout{

    pub fn stdout<T: Display>(lin: &T) {
        println!("{}", lin);
    }
}