pub use termcolor;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

#[macro_export]
macro_rules! prnt {
    ($($arg:tt)*) => {{
        $crate::write_coloured_output(&format!($($arg)*), $crate::termcolor::Color::Cyan);
    }};
}

#[macro_export]
macro_rules! prnth {
    ($($arg:tt)*) => {{
        $crate::write_coloured_output(&format!($($arg)*), $crate::termcolor::Color::Green);
    }};
}

#[macro_export]
macro_rules! prntdbg {
    ($($arg:tt)*) => {{
        $crate::write_coloured_output(&format!($($arg)*), $crate::termcolor::Color::Blue);
    }};
}

#[macro_export]
macro_rules! prnterr {
    ($($arg:tt)*) => {{
        $crate::write_coloured_output(&format!($($arg)*), $crate::termcolor::Color::Red);
    }};
}


#[macro_export]
macro_rules! prntkv {
    // Case 1: Key and value (Display-compatible)
    ($key:expr, $value:expr) => {{
        $crate::write_coloured_key_value(
            $key,
            &$value.to_string(),
            termcolor::Color::Cyan,
            termcolor::Color::Yellow,
        );
    }};

    // Case 2: Key and formatted value
    ($key:expr, $fmt:expr, $($args:tt)+) => {{
        $crate::write_coloured_key_value(
            $key,
            &format!($fmt, $($args)+),
            termcolor::Color::Cyan,
            termcolor::Color::Yellow,
        );
    }};
}


#[macro_export]
macro_rules! prntkve {
    ($key:expr, $value:expr) => {{
        $crate::write_coloured_key_value(
            $key,
            &$value.to_string(),
            termcolor::Color::Red,
            termcolor::Color::Yellow,
        );
    }};

    // Case 2: Key and formatted value
    ($key:expr, $fmt:expr, $($args:tt)+) => {{
        $crate::write_coloured_key_value(
            $key,
            &format!($fmt, $($args)+),
            termcolor::Color::Red,
            termcolor::Color::Yellow,
        );
    }};
}

/// Helper function to write colored text
pub fn write_coloured_output(text: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(color));
    if let Err(e) = stdout.set_color(&color_spec) {
        eprintln!("Error setting color: {}", e);
    }
    if let Err(e) = writeln!(&mut stdout, "{}", text) {
        eprintln!("Error writing to stdout: {}", e);
    }
    if let Err(e) = stdout.reset() {
        eprintln!("Error resetting color: {}", e);
    }
}

/// Helper function to write key-value pairs with different colors
pub fn write_coloured_key_value(key: &str, value: &str, key_colour: Color, value_colour: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // Write the key in green
    let mut key_spec = ColorSpec::new();
    key_spec.set_fg(Some(key_colour));
    if let Err(e) = stdout.set_color(&key_spec) {
        eprintln!("Error setting color: {}", e);
    }
    if let Err(e) = write!(&mut stdout, "{}: ", key) {
        eprintln!("Error writing key: {}", e);
    }

    // Write the value in yellow
    let mut value_spec = ColorSpec::new();
    value_spec.set_fg(Some(value_colour));
    if let Err(e) = stdout.set_color(&value_spec) {
        eprintln!("Error setting color: {}", e);
    }
    if let Err(e) = writeln!(&mut stdout, "{}", value) {
        eprintln!("Error writing value: {}", e);
    }

    if let Err(e) = stdout.reset() {
        eprintln!("Error resetting color: {}", e);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use termcolor::Color;

    #[test]
    fn test_prnth() {
        prnth!("My header");
    }

    #[test]
    fn test_prnt() {
        prnt!("This is a test for prnt!");
    }

    #[test]
    fn test_prnterr() {
        prnterr!("My error");
    }

    #[test]
    fn test_prntdb() {
        prntdbg!("My debug");
    }

    #[test]
    fn test_prntkv() {
        let key = "My int";
        let value = 123;
        prntkv!(key, value); // This will use .to_string() internally
    }

    #[test]
    fn test_prntkv_no_display() {
        let key = "My float";
        let value = 123.87;
        prntkv!(key, "{:.1}",value); // This will use .to_string() internally
    }

    #[test]
    fn test_prntkve() {
        let key = "Bad int";
        let value = 666;
        prntkve!(key, "{:.1}",value); // This will use .to_string() internally
    }

    #[test]
    fn test_prntkve_no_display() {
        let key = "Bad float";
        let value = 123.87;
        prntkve!(key, "{:.1}",value); // This will use .to_string() internally
    }


    #[test]
    fn test_prntkv_custom_colors() {
        write_coloured_key_value("Custom Key", "Custom Value", Color::Cyan, Color::Magenta);
    }
}

