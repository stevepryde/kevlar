use crate::testconfig::TestConfig;
use crate::testresult::{TestResult, TestStatus};
use async_trait::async_trait;

// TODO: update return value to be a Result with custom error type.

pub trait TestCase {
    /// Run the test case and return the TestResult object.
    fn run(&mut self, test_config: TestConfig, test_result: &mut TestResult) -> TestStatus;

}

#[async_trait]
pub trait AsyncTestCase {
    async fn run_async(&mut self, test_config: TestConfig, test_result: &mut TestResult) -> TestStatus;
}
