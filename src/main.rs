use std::{cell::Cell, collections::HashMap, fmt::Write, iter::Map, path::Path, process, sync::{LazyLock, Mutex, MutexGuard, RwLock}};

pub mod services;
pub mod svcman;

// +++++++++
// Constants
// +++++++++

pub const VERSION: &str = "0.0.1";
pub const BUILD_TYPE: &str = "NIGHTLY";

// ++++++++++++
// Public enums
// ++++++++++++

/// Represents an error that may occur during the execution of the program.
//
pub enum ErrorType {
    /// (`SvcSpawnFail`)
    /// Failed to spawn a `Service` with `Service::spawn()` method.
    SvcSpawnFail,

}

// +++++++++++++++++++++++
// Public static variables
// +++++++++++++++++++++++

/// Contains important data, including about `Service` objects - the static variable is created at runtime.
/// Use `read_data()` to perform read operations, and `write_data(...)` for writes.
//
pub static DATA: RwLock<String> = RwLock::new(String::new());

// +++++++++++++++++
// Private functions
// +++++++++++++++++

fn main() {
    // TODO


}

// ++++++++++++++++
// Public functions
// ++++++++++++++++

/// Read from the `DATA` static variable.
/// Returns `Option<String>`, or possibly `None`.
//
pub fn read_data() -> Option<String> {
    //
    let read_op = DATA.try_read();

    match read_op {
        Ok(_) => (),
        Err(_) => return None,
    }

    let rwlrguard  = read_op.unwrap()                   ;
    let data_string         = Option::from(rwlrguard.to_string());

    return data_string;
    //
}

/// Write to the `DATA` static variable.
/// Returns `bool` representing the result.
//
pub fn write_data(data: &str) -> bool {
    //
    let write_lock_op = DATA.try_write();

    match write_lock_op {
        Ok(_) => (),
        Err(_) => return false,
    }

    let mut write_lock  = write_lock_op.unwrap()    ;
    let write_op                = write_lock.write_str(data);

    match write_op {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

//
// pub fn load_svcs(path: &str) {
//     let svcs_path = Path::new(path);
//
//     if !svcs_path.exists() {
//         // TODO
//     }
//     if !svcs_path.is_dir() {
//         // TODO
//     }
//
//     let all_svcs = svcs_path
//         .read_dir()
//         .expect("read_dir call failed")
//         .filter(|entry| {
//             entry
//                 .as_ref()
//                 .expect("Failed to check if the path is a file")
//                 .path()
//                 .is_file()
//         })
//         .filter(|entry| {
//             entry
//                 .as_ref()
//                 .expect("Failed to check if the path has service extension")
//                 .path()
//                 .extension()
//                 == Some(OsStr::new("service"))
//         });
//
//     for entry in all_svcs {
//         if let Ok(an_entry) = entry {
//             println!("Loading {:?}...", an_entry);
//
//             let unit = Unit::from_unitfile(&an_entry.path().as_path());
//             ALL_UNITS
//                 .lock()
//                 .expect("Failed to parse unit file")
//                 .add_unit(unit);
//         }
//     }
// }

