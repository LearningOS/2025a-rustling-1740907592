use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // tests7：设置环境变量 TEST_FOO
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // tests8：启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}