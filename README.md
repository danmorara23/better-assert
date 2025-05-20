# better-assert
#### Assertion helpers with built-in logging powered by [better-logger](https://crates.io/crates/better-logger)
## ⚠️ Is this crate right for you?         
> #### Dependency note        
> **See [better-logger](https://crates.io/crates/better-logger) before you use this crate**   
>
> `better-assert` incorporates `better-logger` as its logging facade. Any project using `better-assert` must do the same.   
> `better-logger` has terminal, file, and network logging functionality.        
### Why you might need it
| Scenario | Pain point with plain `assert!` | How `better-assert` helps |
|----------|---------------------------------|---------------------------|
| **WASM front-end** | An `assert_eq!` fails silently in a user’s browser, you never see it | `log_assert_eq()` sends a log entry to your backend *before* the panic |
| **Headless server / embedded** | A `panic!` crash disappears into `/dev/null` or an unknown serial console | `log_panic()` sends the details to a file, syslog, or over-the-wire sink |
# HOW TO USE
## 😺 ONE: Declare Feature
```rust
/* no default feature enabled (enabling both at once won't compile) */
better-assert = { version = "0.3.1", features = ["native"] }
better-assert = { version = "0.3.1", features = ["wasm"] }
```
## 🦮 TWO: Incorporate better-logger
#### See the [better-logger docs](https://crates.io/crates/better-logger) for the quick-start
```rust
/* Also has no default feature enabled (enabling both at once won't compile) */
better-logger = { version = "1.0.3", features = ["native"] }
better-logger = { version = "1.0.3", features = ["wasm"] }
```
## 💻 THREE: Use
```rust
use better_assert::*;

let left = "value".to_string();
let right = "value".to_string(); 

log_assert_eq(&left, &right); // enabled in all builds
log_assert_ne(&left, &right);

debug_log_assert_eq(&left, &right); // enabled only in debug builds
debug_log_assert_ne(&left, &right);

log_panic(); // always logs, then panics
```
# License
&copy; 2025 Gistyr LLC               
This project, **better-assert**, is dual-licensed under your choice of:
- **Apache License 2.0**  
  See the [LICENSE-APACHE](LICENSE-APACHE) file or view it online at <http://www.apache.org/licenses/LICENSE-2.0>
- **MIT License**  
  See the [LICENSE-MIT](LICENSE-MIT) file or view it online at <http://opensource.org/licenses/MIT>
