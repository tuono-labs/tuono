#[macro_export]
/// Log a message in the terminal using the custom tuono formatter.
/// The messages printed with this macro should inform or
/// guide the user.
///
/// The debug/error messages should be printed using the `tracing` crate
macro_rules! tuono_println {
    ($($arg:tt)*) => {{
        println!("  {}", format!($($arg)*));
    }};
}
