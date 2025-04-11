//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::env;

fn main() {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    // 获取当前时间戳
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 设置环境变量TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", since_the_epoch);
    // 获取当前包的名称
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();

    // 如果是tests8包，设置feature="pass"的条件编译参数
    if pkg_name == "tests8" {
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}

