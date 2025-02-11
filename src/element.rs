use std::cmp::Ordering;
use std::fmt;
use std::io::prelude::*;
use std::io::{stdin, stdout};

/// Sortable item that queries user during comparisons
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Element<T>
where
    T: fmt::Display + Eq,
{
    /// String to be displayed to user
    name: T,
}

impl<T> Element<T>
where
    T: fmt::Display + Eq,
{
    /// Create a new Element with a specified name.
    pub fn new(name: T) -> Element<T> {
        Element { name }
    }
}

impl<T> fmt::Display for Element<T>
where
    T: fmt::Display + Eq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<T> PartialOrd for Element<T>
where
    T: fmt::Display + Eq,
{
    /// Query user to determine order.
    /// User inputs "1" or "2" to stdin to choose the greater element
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        loop {
            println!();
            println!("(1) {}", self.name);
            println!("(2) {}", other.name);
            print!("Your choice: ");
            stdout().flush().unwrap();

            let mut buffer = String::new();
            stdin().lock().read_line(&mut buffer).unwrap();
            match buffer.trim() {
                "1" => return Some(Ordering::Less),
                "2" => return Some(Ordering::Greater),
                _ => {
                    println!("\nInvalid number!");
                }
            };
        }
    }
}

impl<T> Ord for Element<T>
where
    T: fmt::Display + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
