//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::env;

fn main() {
    // 获取当前包的名称
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();

    // 如果是tests8包，设置feature="pass"的条件编译参数
    if pkg_name == "tests8" {
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}

