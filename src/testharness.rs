use crate::testcase::TestCase;
use crate::testconfig::{ConfigType, TestConfig};
use crate::testresult::TestResult;
use flexi_logger::{opt_format, Duplicate};
use log::*;
use std::path::PathBuf;

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
        flexi_logger::Logger::with_env_or_str(&format!("harness=info, {}=debug", test_name))
            .log_to_file()
            .directory(path)
            .format(opt_format)
            .duplicate_to_stderr(Duplicate::All)
            .start()
            .unwrap();
    }

    /// Run the test harness. This will call the main test run() method.
    pub fn run(self, mut my_test: impl TestCase) {
        let test_result = my_test.run(self.config, self.test_result);
        info!("Test Result: {:?}", test_result);
    }
}
