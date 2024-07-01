use std::fmt::Debug;
use std::io::{self, Write};
use std::str::FromStr;

pub struct Input<T> {
    message: String,
    _type: std::marker::PhantomData<T>,
}

impl<T> Input<T>
where
    T: FromStr,
    T::Err: Debug,
{
    pub fn new() -> Self {
        Self {
            message: String::new(),
            _type: std::marker::PhantomData,
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn get_input(self) -> Result<T, String> {
        print!("{}", self.message);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_string();

        match input.parse::<T>() {
            Ok(value) => Ok(value),
            Err(_) => Err(String::from("Not a valid input")),
        }
    }
}

// A convenience function to create the Input object
pub fn input<T>() -> Input<T>
where
    T: FromStr,
    T::Err: Debug,
{
    Input::new()
}
