use log;
use flexi_logger::*;

pub struct Logging {}

impl Logging {
    pub fn init() {
        Logger::try_with_env_or_str("info")
            .unwrap()
            .log_to_file(FileSpec::default())
            .duplicate_to_stdout(Duplicate::Info)
            .append()
            .format(detailed_format)
            .start()
            .unwrap();

    }

    pub fn info(msg: &str) {
        log::info!("{}", msg);
    }

    pub fn error(msg: &str) {
        log::error!("{}", msg);
    }
}
