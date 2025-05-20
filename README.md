# better-assert
#### Rust assertion functions and panic with built in logging
# HOW TO USE
## ⚠️ ONE: Confirm this crate is right for you
#### Look at: [better-logger](https://crates.io/crates/better-logger) before you use this crate
[better-assert](https://crates.io/crates/better-assert) mimics the assertion and panic macros with functions
## 😺 TWO: Declare Feature
```rust
/* no default feature enabled (enabling both at once won't compile) */
better-assert = { version = "0.3.1", features = ["native"] }
better-assert = { version = "0.3.1", features = ["wasm"] }
```
## 💻 THREE: Use
```rust
use better_assert::*;

let left = "value".to_string();
let right = "value".to_string(); 

log_assert_eq(left, right);
log_assert_ne(left, right);
debug_log_assert_eq(left, right);
debug_log_assert_ne(left, right);
log_panic();
```
# ℹ️ INFORMATION
- [better-assert](https://crates.io/crates/better-assert) mimics the assertion and panic macros with functions
- [better-assert](https://crates.io/crates/better-assert) uses [better-logger](https://crates.io/crates/better-logger) as a logging facade
  - Meaning any project that uses [better-assert](https://crates.io/crates/better-assert) must also use [better-logger](https://crates.io/crates/better-logger)  
