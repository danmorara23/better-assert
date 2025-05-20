// better-assert/src/lib.rs

// cargo build --features native
// cargo build --release --features native
// cargo build --target wasm32-unknown-unknown --features wasm
// cargo build --release --target wasm32-unknown-unknown --features wasm

#[cfg(any(feature = "native", feature = "wasm"))]
use better_logger::logger;
#[cfg(any(feature = "native", feature = "wasm"))]
use std::fmt::Debug;
#[cfg(any(feature = "native", feature = "wasm"))]
use std::panic::Location;
#[cfg(any(feature = "native", feature = "wasm"))]
use std::any::type_name;

///0
///1
///2
///3
///4
///5
///6
///7
///8
///9

#[cfg(any(feature = "native", feature = "wasm"))]
#[track_caller]
pub fn log_assert_eq<G>(left: &G, right: &G) 
where
    G: PartialEq + Debug,
{
    if !(left == right) {
        let location = Location::caller();
        let type_of_g = type_name::<G>();
        logger::error!(r#"Assertion Failed: left "{:?}" != right "{:?}" (Location: {}:{}:{}) (Type: {})"#, left, right, location.file(), location.line(), location.column(), type_of_g);
        panic!(r#"Assertion Failed: left "{:?}" != right "{:?}" (Location: {}:{}:{}) (Type: {})"#, left, right, location.file(), location.line(), location.column(), type_of_g);
    }
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[track_caller]
pub fn log_assert_ne<G>(left: &G, right: &G) 
where
    G: PartialEq + Debug,
{
    if !(left != right) {
        let location = Location::caller();
        let type_of_g = type_name::<G>();
        logger::error!(r#"Assertion Failed: left "{:?}" == right "{:?}" (Location: {}:{}:{}) (Type: {})"#, left, right, location.file(), location.line(), location.column(), type_of_g);
        panic!(r#"Assertion Failed: left "{:?}" == right "{:?}" (Location: {}:{}:{}) (Type: {})"#, left, right, location.file(), location.line(), location.column(), type_of_g);
    }
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[cfg(debug_assertions)]
#[track_caller]
pub fn debug_log_assert_eq<G>(left: &G, right: &G) 
where
    G: PartialEq + Debug,
{
    if !(left == right) {
        let location = Location::caller();
        let type_of_g = type_name::<G>();
        logger::error!(r#"Assertion Failed: left "{:?}" != right "{:?}" (Location: {}:{}:{}) (Type: {})"#, left, right, location.file(), location.line(), location.column(), type_of_g);
        panic!(r#"Assertion Failed: left "{:?}" != right "{:?}" (Location: {}:{}:{}) (Type: {})"#, left, right, location.file(), location.line(), location.column(), type_of_g);
    }
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[cfg(not(debug_assertions))]
#[track_caller]
pub fn debug_log_assert_eq<G>(_left: &G, _right: &G) 
where
    G: PartialEq + Debug,
{

}

#[cfg(any(feature = "native", feature = "wasm"))]
#[cfg(debug_assertions)]
#[track_caller]
pub fn debug_log_assert_ne<G>(left: &G, right: &G) 
where
    G: PartialEq + Debug,
{
    if !(left != right) {
        let location = Location::caller();
        let type_of_g = type_name::<G>();
        logger::error!(r#"Assertion Failed: left "{:?}" == right "{:?}" (Location: {}:{}:{}) (Type: {})"#, left, right, location.file(), location.line(), location.column(), type_of_g);
        panic!(r#"Assertion Failed: left "{:?}" == right "{:?}" (Location: {}:{}:{}) (Type: {})"#, left, right, location.file(), location.line(), location.column(), type_of_g);
    }
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[cfg(not(debug_assertions))]
#[track_caller]
pub fn debug_log_assert_ne<G>(_left: &G, _right: &G) 
where
    G: PartialEq + Debug,
{

}

#[cfg(any(feature = "native", feature = "wasm"))]
#[track_caller]
pub fn log_panic() {
    let location = Location::caller();
    logger::error!(r#"Panic (Location: {}:{}:{})"#, location.file(), location.line(), location.column());
    panic!(r#"Panic (Location: {}:{}:{})"#, location.file(), location.line(), location.column());
}