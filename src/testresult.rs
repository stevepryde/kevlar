#[derive(Debug)]
pub enum TestStatus {
    // The test passed.
    Passed,
    // The test failed due to a bug either in the SUT or the test.
    Failed,
    /// The test failed due to an issue that is not going to be fixed soon.
    /// It's basically a pass but we still want to know when the issue is encountered.
    KnownFailure,
    // The test was skipped either due to manual intervention or unmet requirements.
    Skipped,
}

impl TestStatus {
    pub fn get_precedence(&self) -> u8 {
        match self {
            TestStatus::Passed => 0,
            TestStatus::KnownFailure => 1,
            TestStatus::Failed => 2,
            TestStatus::Skipped => 3,
        }
    }
}

/// The TestResult struct contains details about the test including the
/// test name and current status. An instance of TestResult is owned and
/// provided by the Test Harness to the test's run*() method.
#[derive(Debug)]
pub struct TestResult {
    name: String,
    status: TestStatus,
}

impl TestResult {
    /// Create new default test result.
    ///
    /// The TestResult object will be automatically instantiated by the
    /// Test Harness and supplied to your test. You shouldn't need to create
    /// it yourself.
    pub fn new(test_name: &str) -> Self {
        TestResult {
            name: test_name.to_owned(),
            status: TestStatus::Passed,
        }
    }

    /// Update test status. This will only take effect if the new status has a
    /// higher precedence value than the existing status.
    ///
    /// ```
    /// test_result.set_status(TestStatus::Failed);
    /// // Underlying status is now TestStatus::Failed.
    /// test_result.set_status(TestStatus::KnownFailure);
    /// // Underlying status remains as Failed because KnownFailure has lower
    /// // precedence.
    /// ```
    pub fn set_status(&mut self, new_status: TestStatus) {
        if new_status.get_precedence() > self.status.get_precedence() {
            self.status = new_status;
        }
    }
}
