use crate::testcase::{TestCase, AsyncTestCase};
use crate::testconfig::{ConfigType, TestConfig};
use crate::testresult::TestResult;
use log::*;
use std::path::PathBuf;
use std::time::Instant;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct TestHarness {
    config: TestConfig,
    test_result: TestResult,
}

impl TestHarness {
    /// Initialize the test harness.
    pub fn new(test_name: &str, config: ConfigType) -> Self {
        let config = TestConfig::load(test_name, config);
        TestHarness::init_logging(test_name, &config.path);
        info!("Kevlar Test Harness :: {}", VERSION);
        info!("-----------------------------");
        // TODO: collect system info and log it here.
        TestHarness {
            config,
            test_result: TestResult::new(test_name),
        }
    }

    /// Set up logging both to stderr and to a file.
    fn init_logging(test_name: &str, path: &PathBuf) {
        let mut log_file = path.clone();
        log_file.push("test.log");

        fern::Dispatch::new()
            .level(log::LevelFilter::Off)
            .level_for("kevlar", log::LevelFilter::Debug)
            .level_for(format!("{}", test_name), log::LevelFilter::Debug)
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .chain(std::io::stdout())
            .chain(
                fern::log_file(log_file)
                    .expect(&format!("Error writing to log file: {}", path.display())),
            )
            .apply()
            .expect("Error setting up logging");
    }

    /// Run the test harness. This will call the main test run() method.
    pub fn run(self, mut my_test: impl TestCase) {
        let timer = Instant::now();
        let mut test_result = self.test_result;
        my_test.run(self.config, &mut test_result);
        info!(
            "Test completed in {:.3} seconds",
            timer.elapsed().as_secs_f64()
        );
        info!("Test Result: {:?}", test_result);
    }

    pub async fn run_async(self, mut my_test: impl AsyncTestCase) {
        let timer = Instant::now();
        let mut test_result = self.test_result;
        let test_status = my_test.run_async(self.config, &mut test_result).await;
        test_result.set_status(test_status);
        info!(
            "Test completed in {:.3} seconds",
            timer.elapsed().as_secs_f64()
        );
        info!("Test Result: {:?}", test_result);
    }
}
