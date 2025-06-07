use std::time::SystemTime;

pub fn init() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
                .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target().rsplit("::").next().unwrap_or(record.target()),
                message
            ))
        })
        .apply()
        .expect("Failed to configure logging with fern");
    log::info!("Logging configured with fern.");
}