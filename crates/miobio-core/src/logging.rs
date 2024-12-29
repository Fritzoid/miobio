use log;
use flexi_logger::Duplicate;
use flexi_logger::FileSpec;
use flexi_logger::Logger;

pub struct Logging {}

impl Logging {
    pub fn init() {
        Logger::try_with_env_or_str("info")
            .unwrap()
            .log_to_file(FileSpec::default())
            .duplicate_to_stdout(Duplicate::Info)
            .append()
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
