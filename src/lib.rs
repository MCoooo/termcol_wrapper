pub use termcolor;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

#[macro_export]
macro_rules! prnt {
    ($($arg:tt)*) => {{
        $crate::write_colored_output(&format!($($arg)*), $crate::termcolor::Color::Cyan);
    }};
}

#[macro_export]
macro_rules! prntdbg {
    ($($arg:tt)*) => {{
        $crate::write_colored_output(&format!($($arg)*), $crate::termcolor::Color::Blue);
    }};
}

#[macro_export]
macro_rules! prnterr {
    ($($arg:tt)*) => {{
        $crate::write_colored_output(&format!($($arg)*), $crate::termcolor::Color::Red);
    }};
}

#[macro_export]
macro_rules! prntkv {
    ($key:expr, $value:expr) => {{
        $crate::write_colored_key_value($key, $value);
    }};
}

/// Helper function to write colored text
pub fn write_colored_output(text: &str, color: Color) {
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
pub fn write_colored_key_value(key: &str, value: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // Write the key in green
    let mut key_spec = ColorSpec::new();
    key_spec.set_fg(Some(Color::Green));
    if let Err(e) = stdout.set_color(&key_spec) {
        eprintln!("Error setting color: {}", e);
    }
    if let Err(e) = write!(&mut stdout, "{}: ", key) {
        eprintln!("Error writing key: {}", e);
    }

    // Write the value in yellow
    let mut value_spec = ColorSpec::new();
    value_spec.set_fg(Some(Color::Yellow));
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

    #[test]
    fn test_prnt() {
        prnt!("This is a test for prnt!");
    }
}
