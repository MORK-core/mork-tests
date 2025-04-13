use std::env;
fn main() {
    // 根据不同的二进制目标设置不同值
    let target = env::var("TARGET_BIN_NAME").unwrap_or_default();

    match target.as_str() {
        "test_hello1" => {
            println!("cargo:warning=正在设置 STATIC_HEAP_SIZE 给目标: {}", target);
            println!("cargo:rustc-env=STATIC_HEAP_SIZE=1"); // 2MB
        }
        "test_hello2" => {
            println!("cargo:warning=正在设置 STATIC_HEAP_SIZE 给目标: {}", target);
            println!("cargo:rustc-env=STATIC_HEAP_SIZE=1"); // 1MB
        }
        _ => {}
    }
    let t = env::var("STATIC_HEAP_SIZE").unwrap_or_default();
    println!("cargo:warning=正在设置 STATIC_HEAP_SIZE 给目标: {}, ,,,{},,", target, t);

    println!("cargo:rerun-if-changed=build.rs");
}