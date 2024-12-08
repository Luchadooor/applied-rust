//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```   
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for green.
/// # Examples:
/// ```   
/// use cli_utils::colors::*;
/// println!("{}", green("Green"));
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for blue.
/// # Examples:
/// ```   
/// use cli_utils::colors::*;
/// println!("{}", blue("Blue"));
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for bold.
/// # Examples:
/// ```   
/// use cli_utils::colors::*;
/// println!("{}", bold("Bold"));
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for neutral formatting.
/// # Examples:
/// ```   
/// use cli_utils::colors::*;
/// println!("{}", reset("reset"));
/// ```
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for neutral formatting.
/// # Examples:
/// ```   
/// use cli_utils::colors::*;
/// println!("{}", undefined("undefined"));
/// ```
pub fn undefined(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}




/// All available "Colors"
/// # Example:
/// ```
///     pub fn paint(&mut self) {
///        match self.color {
///            Color::Red => self.colorized = red(&self.string),
///            Color::Green => self.colorized = green(&self.string),
///            Color::Blue => self.colorized = blue(&self.string),
///            Color::Bold => self.colorized = bold(&self.string),
///       };
///     }
/// ```
pub enum Color{
    Undefined,
    Red,
    Green,
    Blue,
    Bold,
}

/// Container that holds a string and its formatted version
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}

/// Implementation of ColorString
/// # Example:
/// ```
/// Pending
/// ```
impl ColorString {
    // create a method that will use the string and color fields to create a colorized string and assign it to the colorized field
    /// Apply the color to the stirng.
    /// ColorString has to be set mutable for this.
    /// # Example:
    /// ```
    /// fn main() {
    ///     println!("Hello, world!");
    ///     let text: String = "Hello".to_string();
    ///     let current_color: cli_utils::colors::Color = cli_utils::colors::Color::Red;
    ///     let mut color_string = cli_utils::colors::ColorString {
    ///         color: current_color, 
    ///         string: text, 
    ///         colorized: String::new(),
    ///     };
    ///     color_string.paint();
    ///     println!("Text: {:?}", color_string.colorized);
    /// }
    /// ```
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
            Color::Undefined => self.colorized = undefined(&self.string),
        };
    }

    /// Undo the paint()
    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }

    /// Clears all information inside the ColorString instance
    pub fn clear(&mut self) {
        self.string = String::new();
        self.colorized = String::new();
        self.color = Color::Undefined;
    }

}
