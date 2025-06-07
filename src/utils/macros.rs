#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => {{
        use crossterm::ExecutableCommand;
        
        std::io::stdout()
            .execute(crossterm::terminal::LeaveAlternateScreen).unwrap()
            .execute(crossterm::cursor::Show).unwrap();

        crossterm::terminal::disable_raw_mode().unwrap();

        log::error!($($arg)+);

        std::process::exit(1);
    }};
}