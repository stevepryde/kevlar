#[derive(Debug)]
pub enum TestStatus {
    Passed,
    Failed,
    KnownFailure,
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

#[derive(Debug)]
pub struct TestResult {
    name: String,
    status: TestStatus,
}

impl TestResult {
    /// Create new default test result.
    pub fn new(test_name: &str) -> Self {
        TestResult {
            name: test_name.to_owned(),
            status: TestStatus::Passed,
        }
    }

    /// Update test status. This will only take effect if the new status has a
    /// higher precedence value than the existing status.
    pub fn set_status(&mut self, new_status: TestStatus) {
        if new_status.get_precedence() > self.status.get_precedence() {
            self.status = new_status;
        }
    }
}
