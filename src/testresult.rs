use log::*;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum TestStatus {
    /// The test passed.
    Passed,
    /// The test failed due to a bug either in the SUT or the test.
    Failed,
    /// The test failed due to an issue that is not going to be fixed soon.
    /// It's basically a pass but we still want to know when the issue is encountered.
    KnownFailure,
    /// The test was skipped either due to manual intervention or unmet requirements.
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

impl fmt::Display for TestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TestStatus::Passed => "PASSED",
                TestStatus::Failed => "FAILED",
                TestStatus::KnownFailure => "KNOWNFAIL",
                TestStatus::Skipped => "SKIPPED",
            }
        )
    }
}

#[derive(Debug)]
pub enum TestArtifactType {
    Log,
    Data,
    PDF,
    Image,
    Video,
    Unknown,
}

/// Test Artifacts are files that your test generates or obtains during its
/// life cycle. These can be passed to the [TestHarness](struct.TestHarness.html) as
/// part of a [TestEvent](struct.TestEvent.html).
/// See [TestRecord::add_event()](struct.TestRecord.html#method.add_event) for more details.
#[derive(Debug)]
pub struct TestArtifact {
    filename: PathBuf,
    artifact_type: TestArtifactType,
    label: String,
    description: String,
}

impl TestArtifact {
    /// Create a new TestArtifact. Also specify a label to identify it.
    pub fn new(filename: PathBuf, label: &str) -> Self {
        TestArtifact {
            filename,
            artifact_type: TestArtifactType::Unknown,
            label: label.to_owned(),
            description: String::new(),
        }
    }

    /// Specify the type of artifact.
    ///
    /// Example:
    /// ```
    /// let artifact = TestArtifact::new(filename, "Log File")
    ///     .with_type(TestArtifactType::Log);
    /// ```
    pub fn with_type(mut self, artifact_type: TestArtifactType) -> Self {
        self.artifact_type = artifact_type;
        self
    }

    /// Add a description for the artifact.
    ///
    /// Example:
    /// ```
    /// let artifact = TestArtifact::new(filename, "Screen capture")
    ///     .with_description("The user profile page, with details");
    /// ```
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_owned();
        self
    }
}

/// A TestEvent is any event that you want to highlight during the test.
/// Typically this will be a test failure but you might also want to capture
/// other events also.
#[derive(Debug)]
pub struct TestEvent {
    status: TestStatus,
    description: String,
    artifacts: Vec<TestArtifact>,
}

impl TestEvent {
    /// Create a new TestEvent.
    pub fn new(status: TestStatus) -> Self {
        TestEvent {
            status,
            description: String::new(),
            artifacts: Vec::new(),
        }
    }

    /// Supply a description to the TestEvent. Supports chaining.
    ///
    /// Example:
    /// ```
    /// TestEvent::new(TestStatus::Failed).with_description("Something went wrong")
    /// ```
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_owned();
        self
    }

    /// Supply an artifact to the TestEvent. Supports chaining. You can add
    /// multiple artifacts by chaining this several times.
    ///
    /// Example:
    /// ```
    /// let event = TestEvent::new(TestStatus::Failed)
    ///     .with_description("Something went wrong")
    ///     .with_artifact(artifact1)
    ///     .with_artifact(artifact2);
    /// ```
    pub fn with_artifact(mut self, artifact: TestArtifact) -> Self {
        self.add_artifact(artifact);
        self
    }

    /// Set the description of this event. This will override any existing
    /// description if one was already specified.
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_owned();
    }

    /// Add an artifact to this event.
    pub fn add_artifact(&mut self, artifact: TestArtifact) {
        self.artifacts.push(artifact);
    }
}

impl fmt::Display for TestEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut desc = self.status.to_string();
        if !self.description.is_empty() {
            desc += &format!(" :: {}", self.description);
        }
        if !self.artifacts.is_empty() {
            desc += &format!(
                " :: Captured {} {}",
                self.artifacts.len(),
                match self.artifacts.len() {
                    1 => "artifact",
                    _ => "artifacts",
                }
            );
        }

        write!(f, "{}", desc)
    }
}

pub type TestResult = Result<(), TestEvent>;

/// The TestResult struct contains details about the test including the
/// test name and current status. An instance of TestResult is owned and
/// provided by the Test Harness to the test's run*() method.
#[derive(Debug)]
pub struct TestRecord {
    name: String,
    status: TestStatus,
    events: Vec<TestEvent>,
}

impl TestRecord {
    /// Create new default test result.
    ///
    /// The TestResult object will be automatically instantiated by the
    /// Test Harness and supplied to your test. You shouldn't need to create
    /// it yourself.
    pub fn new(test_name: &str) -> Self {
        TestRecord {
            name: test_name.to_owned(),
            status: TestStatus::Passed,
            events: Vec::new(),
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

    /// Get the test status.
    pub fn get_status(&self) -> &TestStatus {
        &self.status
    }

    /// Add a [TestEvent](struct.TestEvent.html) to this TestRecord.
    ///
    /// Lots of things can happen during a test. Keep track of it all by
    /// adding events to your test record as you go.
    ///
    /// Events will be logged, and will automatically update the test status.
    pub fn add_event(&mut self, event: TestEvent) {
        match event.status {
            TestStatus::Passed => info!("[EVENT] {}", event),
            TestStatus::Failed => error!("[EVENT] {}", event),
            TestStatus::KnownFailure => warn!("[EVENT] {}", event),
            TestStatus::Skipped => warn!("[EVENT] {}", event),
        }

        self.set_status(event.status.clone());
        self.events.push(event);
    }

    /// Apply the specified [TestResult](type.TestResult.html) to this
    /// TestRecord.
    ///
    /// This will add the event to the TestRecord which will in turn update
    /// the test status. Any artifacts will be processed by the TestHarness.
    pub fn apply_result(&mut self, result: TestResult) {
        match result {
            Ok(_) => self.set_status(TestStatus::Passed),
            Err(e) => self.add_event(e),
        }
    }
}
