use crate::testconfig::TestConfig;
use crate::testresult::{TestRecord, TestResult};
use async_trait::async_trait;

/// Implement the TestCase trait in order to run your test synchronously.
/// The run() method will be called by the Test Harness.
///
/// ```
/// #[derive(Default)]
/// struct MyTest;
///
/// impl TestCase for MyTest {
///     fn run(&mut self, test_config: TestConfig, test_record: &mut TestRecord) -> TestResult {
///         info!("Do something interesting");
///         Ok(())
///     }
/// }
/// ```
pub trait TestCase {
    /// Run the test case and return the TestStatus code.
    fn run(&mut self, test_config: TestConfig, test_record: &mut TestRecord) -> TestResult;
}

/// Implement the AsyncTestCase trait in order to run your test asynchronously.
/// The run_async() method will be called by the Test Harness.
///
/// ```
/// #[derive(Default)]
/// struct MyTest;
///
/// #[async_trait]
/// impl AsyncTestCase for MyTest {
///     async fn run_async(&mut self, test_config: TestConfig, test_record: &mut TestRecord) -> TestResult {
///         info!("Do something interesting");
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait AsyncTestCase {
    /// Run the test case and return the TestStatus code.
    async fn run_async(&mut self, test_config: TestConfig, test_record: &mut TestRecord) -> TestResult;
}
