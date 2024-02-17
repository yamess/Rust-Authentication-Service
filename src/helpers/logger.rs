use chrono::Utc;
use fern::colors::{Color, ColoredLevelConfig};
use std::fs::File;

use std::io::Write;
pub fn setup_logger(file_path: &str) {
    let target = Box::new(File::create(file_path).expect("Can't create file"));
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Magenta)
        .error(Color::Red)
        .trace(Color::BrightBlack)
        .warn(Color::Yellow);

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(Box::new(target)))
        .target(env_logger::Target::Stdout)
        .filter(None, log::LevelFilter::Debug)
        .format(move |buf, record| {
            writeln!(
                buf,
                "[{datetime} - {level} - Thread {thread} - {module} - line {line}]: {message}",
                datetime = Utc::now().format("%Y-%m-%d %H:%M:%S"),
                level = colors.color(record.level()),
                thread = std::thread::current()
                    .name()
                    .unwrap_or("unnamed")
                    .to_uppercase(),
                module = record.module_path().unwrap(),
                line = record.line().unwrap(),
                message = record.args()
            )
        })
        .init();
}
