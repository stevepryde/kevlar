//! Kevlar is a light-weight Test Harness that provides basic test features
//! such as logging, setting up a test workspace, and managing the test
//! result object, so that you can focus on writing your tests and the
//! abstractions / support libraries that go along with them.
//!
//! This crate is designed for writing test suites for testing other software.
//!
//! If you just want to write unit tests, it is recommended to use Rust's
//! [built-in testing features](https://doc.rust-lang.org/book/ch11-00-testing.html) instead.
//!
//! ```rust
//! use kevlar::{ConfigType, TestConfig, TestHarness, TestStatus, TestResult, AsyncTestCase};
//! use std::path::PathBuf;
//! use async_trait::async_trait; // Required until async is supported in traits
//! use log::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let harness = TestHarness::new(
//!         "kevlar_example",
//!         ConfigType::File(PathBuf::from("./config.json")),
//!     );
//!     harness.run_async::<MyTest>().await;
//! }
//!
//! #[derive(Default)]
//! struct MyTest;
//!
//! #[async_trait]
//! impl AsyncTestCase for MyTest {
//!     async fn run_async(&mut self, test_config: TestConfig, test_result: &mut TestResult) -> TestStatus {
//!         info!("Do something interesting!");
//!         TestStatus::Passed
//!     }
//!}
//!```

pub mod testcase;
pub mod testconfig;
pub mod testharness;
pub mod testresult;

// Convenience re-exports.
pub use testcase::{AsyncTestCase, TestCase};
pub use testconfig::{ConfigType, TestConfig};
pub use testharness::TestHarness;
pub use testresult::{TestResult, TestStatus};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
