use crate::testconfig::TestConfig;
use crate::testresult::TestResult;

pub trait TestCase {
    /// Run the test case and return the TestResult object.
    fn run(&mut self, test_config: TestConfig, test_result: TestResult) -> TestResult;
}
