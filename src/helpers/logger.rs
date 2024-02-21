use actix_web::web;
use chrono::Utc;
use fern::colors::{Color, ColoredLevelConfig};
use std::sync::Arc;

pub fn setup_logger(file_path: &str) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .trace(Color::BrightBlack)
        .debug(Color::Blue)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::BrightRed);

    let file_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{datetime} - {level} - Thread {thread} - {target}: {line}]: \
                {message}",
                datetime = Utc::now().format("%Y-%m-%d %H:%M:%S"),
                level = record.level(),
                thread = std::thread::current()
                    .name()
                    .unwrap_or("unnamed")
                    .to_uppercase(),
                target = record.target(),
                line = record.line().unwrap(),
                message = message
            ))
        })
        .chain(fern::log_file(file_path).unwrap());

    let console_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{datetime} - {level} - Thread {thread} - {target}: {line}]: \
                {message}",
                datetime = Utc::now().format("%Y-%m-%d %H:%M:%S"),
                level = colors.color(record.level()),
                thread = std::thread::current()
                    .name()
                    .unwrap_or("unnamed")
                    .to_uppercase(),
                target = record.target(),
                line = record.line().unwrap(),
                message = message
            ))
        })
        .chain(std::io::stdout());

    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(console_dispatcher)
        .chain(file_dispatcher)
        .apply()?;
    Ok(())
}
