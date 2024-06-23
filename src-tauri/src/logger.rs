use log4rs::append::rolling_file::policy::compound::CompoundPolicy;

use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender},
        rolling_file::policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

use crate::util::get_log_file_path;


// %d %-5p %X {THREAD_UUID} %c{0} - %m%n

const TRIGGER_FILE_SIZE: u64 = 2 * 1024;
const LOG_FILE_COUNT: u32 = 3;
// const ARCHIVE_PATTERN: &str = "/tmp/archive/foo.{}.log";



pub fn init_logger() -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;

    let binding = get_log_file_path();
    let file_path = binding.as_str();

    let stdout: ConsoleAppender = ConsoleAppender::builder().build();

    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder()
        .base(0) // Default Value (line not needed unless you want to change from 0 (only here for demo purposes)
        .build(file_path, LOG_FILE_COUNT) // Roll based on pattern and max 3 archive files
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    .build(file_path, Box::new(policy))
    .unwrap();

    let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .appender(
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(level)))
            .build("stderr", Box::new(stdout)),
    )
    .build(
        Root::builder()
            .appender("logfile")
            .appender("stderr")
            .build(LevelFilter::Trace),
    )
    .unwrap();

    let _handle = log4rs::init_config(config)?;

    Ok(())

    // let requests = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
    //     .build("log/requests.log")
    //     .unwrap();

    // let config = Config::builder()
    //     .appender(Appender::builder().build("stdout", Box::new(stdout)))
    //     .appender(Appender::builder().build("requests", Box::new(requests)))
    //     .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
    //     .logger(Logger::builder()
    //         .appender("requests")
    //         .additive(false)
    //         .build("app::requests", LevelFilter::Info))
    //     .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
    //     .unwrap();

    // let handle = log4rs::init_config(config).unwrap();
    // handle
}