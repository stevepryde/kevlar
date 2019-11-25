# Kevlar Test Harness

A test harness for writing full-featured integration / regression tests in Rust.

Its main goals are to provide all of the basic test harness functionality to bootstrap your
test case, so that you can just focus on writing the tests.

This includes:
- Logging (already set up with a unique workspace directory created for your test)
- Configuration parameters
- Setting up a test workspace directory
- Managing artifacts (provide methods to attach artifacts to the test result)
- Submit test results to external webhook easily
- Derive test status code from error return values
- Useful utilities

**NOTE** This project is WIP and not ready for production use.

## Doesn't Rust already support automated tests via "cargo test" ?

Yes it does. However this crate is not intended to replace these kinds of tests.
For unit tests and small, quick tests of your code, you should continue 
to use Rust's built-in test facilities (see the official documentation
[here](https://doc.rust-lang.org/book/ch11-00-testing.html)).

This crate is intended for larger-scale testing, particularly integration 
testing, for things like testing web applications using
Selenium, or testing other software or even hardware platforms.
Use it to create a robust test suite for a separate software or 
hardware application.

Rather than spend time writing all of the boilerplate to provide facilities 
such as logging and test result capture, this crate allows you
to focus on writing your test code and the related abstractions for your 
System-Under-Test.


## Example usage

**NOTE** It is still WIP. I plan to remove a lot of boilerplate and shrink 
this down as much as possible. Both synchronous and asyncronous tests are supported.

I will be experimenting with the syntax for this quite a lot in the coming weeks/months, until
I come up with something that feels ergonomic to write at scale.

```rust
use kevlar::{ConfigType, TestConfig, TestHarness, TestStatus, TestResult, AsyncTestCase};
use std::path::PathBuf;
use async_trait::async_trait; // Required until async is supported in traits
use log::*;

#[tokio::main]
async fn main() {
    let harness = TestHarness::new(
        "kevlar_example",
        ConfigType::File(PathBuf::from("./config.json")),
    );
    harness.run_async::<MyTest>().await;
}

#[derive(Default)]
struct MyTest;

#[async_trait]
impl AsyncTestCase for MyTest {
    async fn run_async(&mut self, test_config: TestConfig, test_result: &mut TestResult) -> TestStatus {
        info!("Do something interesting!");
        TestStatus::Passed
    }
}
```

The config.json file only needs to specify the path for now:
```json
{"path": "/tmp/kevlar-tests/"}
```

## Launching individual tests

Typically Automated Test Frameworks launch tests by specifying the 
test name via a CLI, and then dynamically loading the corresponding
module and instantiating the test object within it on the fly.

Rust is a statically-typed language and does not have the reflection
or dynamic loading capabilities that a language such as Python has.
The closest equivalent would be dynamically linked libraries (DLLs). 

However, loading dynamic modules means the modules themselves need to 
contain a lot of boilerplate in order to expose the C-style FFI.
You end up adding complexity essentially just to avoid including the 
test harness setup code in each test. But this only matters if your 
test harness is very large.

The solution proposed by this crate is to provide the Test Harness
as a light-weight add-on that you can use to bootstrap your tests in
just a few lines of code. After that, your tests are just 
straightforward binaries and can be run directly from the command-line.

I'm interested in exploring the possibility of also providing basic 
CLI option parsing facilities in the Test Harness as well. However
the primary way to provide custom options to tests will be via
a config.json file or via environment variables.

Another area of interest is how to group tests together into a suite,
so that they can be run together. I've always preferred to keep the 
test framework and test runner infrastructure separate, so that the 
job of parallelisation and managing test runs is beyond the scope of 
the Test Harness itself.

Rust allows crates to build multiple binaries, which may be an option
to explore for housing multiple tests in the same crate. Otherwise
each test would need to have its own crate, and be independently built.
This is not so terrible, but does add some complexity to your build 
system. Any reduction in complexity it good so I will look at ways 
to simplify this.

This crate should be considered experimental for the foreseeable future
and later versions will very likely contain breaking changes.

Stability is planned for v1.0, if/when I get there.
