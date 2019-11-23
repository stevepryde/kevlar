# Kevlar Test Harness

A test harness for writing integration / regression tests in Rust.

Its main goals are to provide all of the basic test harness functionality to bootstrap your
test case, so that you can just focus on writing the tests.

This includes:
- Logging
- Configuration parameters
- Setting up a test workspace directory
- Managing artifacts (provide methods to attach artifacts to the test result)
- Submit test results to external webhook easily
- Derive test status code from error return values
- Useful utilities

**NOTE** This project is WIP and not ready for production use.

## Example usage

This is a complete example (see <https://github.com/stevepryde/kevlar-example>).

**NOTE** It is still WIP. I plan to remove a lot of boilerplate and shrink 
this down as much as possible. Both synchronous and asyncronous tests are supported.

```rust
use kevlar::{ConfigType, TestConfig, TestHarness, TestStatus, TestResult, AsyncTestCase};
use std::path::PathBuf;
use async_trait::async_trait;
use fantoccini::Client;
use log::*;

#[tokio::main]
async fn main() -> Result<(), String>{
    let harness = TestHarness::new(
        "kevlar_example",
        ConfigType::File(PathBuf::from("./config.json")),
    );
    let my_test = MyTest::default();
    harness.run_async(my_test).await;
    Ok(())
}

#[derive(Default)]
struct MyTest {
    internal_data: String,
}

#[async_trait]
impl AsyncTestCase for MyTest {
    async fn run_async(&mut self, _test_config: TestConfig, _test_result: &mut TestResult) -> TestStatus {
        // TODO: update return value to be a Result with custom error type that also understands CmdError
        match self.do_stuff().await {
            Ok(_) => TestStatus::Passed,
            Err(_) => TestStatus::Failed
        }
    }
}

impl MyTest {
    async fn do_stuff(&mut self) -> Result<(), fantoccini::error::CmdError> {
        debug!("Load browser at localhost:4444");
        // Some example code taken from fantoccini project.
        let mut c = Client::new("http://localhost:4444").await.expect("failed to start session");
        info!("Load wikipedia");
        c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
        self.internal_data = c.current_url().await?.to_string();
        assert_eq!(self.internal_data, "https://en.wikipedia.org/wiki/Foobar");
        c.close().await?;
        Ok(())
    }
}
```

## TODO

- Selenium support
- SUT setup/teardown providers
- junit file support

and much more!
