#[derive(Debug)]
pub enum TestStatus {
    Passed,
    Failed,
    KnownFailure,
    Skipped,
}

#[derive(Debug)]
pub struct TestResult {
    name: String,
    status: TestStatus,
}

impl TestResult {
    pub fn new(test_name: &str) -> Self {
        TestResult {
            name: test_name.to_owned(),
            status: TestStatus::Passed,
        }
    }
}
