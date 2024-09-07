use std::borrow::Borrow;
use std::cell::Cell;
use std::fs::{self, File, Permissions};
use std::os::linux::process;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::os::unix::process;
use std::path::Path;
use std::process;
use std::sync::Arc;

// ++++++++++++
// Public enums
// ++++++++++++

//
#[derive(PartialEq, Eq, PartialOrd, Ord)]
//
/// Represents the type of a `Service` object.
//
pub enum ServiceType {
	/// Service which is managed by the system for all of its lifetime. Returns its state to the system on exit.
	Regular,
	/// Similar to `Regular`, but allows the `Service` to have certain `ServiceFlags` set.
	Protected,
	/// `Service` is only started by the system, but not managed by it. Returns the result of a startup attempt immediately after.
	TryOnce,
}

//
#[derive(PartialEq, Eq, PartialOrd, Ord)]
//
/// Represents the possible flags set for a `Service` object.
//
pub enum ServiceFlags {
	/// No flags will be set for this `Service`, except for this one.
	None,
	/// Disallow the `Service` process from gaining new privileges.
	NoGainPrivs,
	/// Disallow the `Service` process and its descendants from being stopped/terminated/killed.
	NoKill,
}

//
#[derive(PartialEq, Eq, PartialOrd, Ord)]
//
/// Represents the state of a `Service` object.
//
pub enum ServiceState {
	Fresh,
	Active,
	Restarting,
	Stopped,
	Errored,
}

// ++++++++++++++++++++++++++
// Public structs & impl code
// ++++++++++++++++++++++++++

/// Represents a service and it's parameters.
//
pub struct Service {
	//
	/// Description of this `Service`.
	pub description                                        : Option<String>      ,
	/// Type of `Service` which can be one of the following:
	/// `Regular / Protected / TryOnce`
	pub svctype                                            : ServiceType         ,
	/// The specified `Service` is loaded before this.
	pub before                                             : Option<String>      ,
	/// The specified `Service` is loaded after this.
	pub after                                              : Option<String>      ,
	//
	// TODO
	// pub wants                                           : Option<Arc<Service>>,
	//
	/// The specified file is executed at start.
	pub action_start                                       : String              ,
	/// The specified file is executed if the `Service` is restarted.
	pub action_restart                                     : Option<String>      ,
	/// The specified file is executed when the `Service` is stopped.
	pub action_stop                                        : Option<String>      ,
	/// Flags set for this `Service`.
	pub flags                                              : Option<ServiceFlags>,
	/// Current value of `ServiceState` for this `Service`.
	pub state                                              : Cell<ServiceState>  ,
	//
}

impl Service {
	//
	// TODO
	// pub fn from_file(path: &Path) -> Service {
	//     let file = 0;
	// }
	//

	/// Spawns a new `Service` from the provided arguments.
	/// *Note that this will always return a `Service` object, even if a valid `Service` is not spawned!*
	//
	pub fn spawn(
		//
		description   : Option<String>      ,
		svctype       : ServiceType         ,
		before        : Option<String>      ,
		after         : Option<String>      ,
		action_start  : String              ,
		action_restart: Option<String>      ,
		action_stop   : Option<String>      ,
		flags         : Option<ServiceFlags>,
		//
	) -> Service {
		//
		let new = Service {
			description                          ,
			svctype                              ,
			before                               ,
			after                                ,
			action_start                         ,
			action_restart                       ,
			action_stop                          ,
			flags                                ,
			state: Cell::new(ServiceState::Fresh),
		};
		//
		return new;
	}

	/// Starts the `Service` if it is valid.
	/// Returns a `bool` representing the result.
	//
	pub fn start(self) -> bool {
		let op_result = start_svc(&self);
	}

	/// Stops the `Service` if it is currently active.
	/// Returns a `bool` representing the result.
	//
	pub fn stop(self) -> bool {
		// TODO
	}

	/// Restarts the `Service` if it currently active.
	/// Returns a `bool` representing the result.
	//
	pub fn restart(self) -> bool {
		// TODO
	}

	/// **WARNING: Dangerous operation!**
	///
	/// Kills the `Service` immediately if possible.
	/// Returns a `bool` representing the result.
	//
	pub fn kill(self) -> bool {
		// TODO
	}
}

// +++++++++++++++++
// Private functions
// +++++++++++++++++

/// Starts the provided `Service`. Returns a `u8` representing the result.
/// *Possible `return` values:*
/// (`0`) *Success*
/// (`1`) *Invalid flags*
/// (`2`) *Already started*
/// (`3`) *Invalid executable*
/// (`4`) *Exec failed*
//
fn start_svc(service: &Service) -> u8 {
	//
	// Struct field values
	//
	let flags = &service.flags.unwrap_or(ServiceFlags::None);

	if flags == &ServiceFlags::None {
		//
		if flags == &ServiceFlags::NoGainPrivs || flags == &ServiceFlags::NoKill {
			return 1
		}
	}
	//
	let state = &service.state.into_inner();

	if state != &ServiceState::Fresh {
		if state != &ServiceState::Stopped {
			return 2
		}
	}

	// Info
	//
	let desc                  = &*service.description.unwrap_or(String::from(""))   ;
	let stype          = &service.svctype                                          ;
	let before                = &*service.before.unwrap_or(String::from(""))        ;
	let after                 = &*service.after.unwrap_or(String::from(""))         ;
	let start                 = &*service.action_start                                    ;
	let stop                  = &*service.action_stop.unwrap_or(String::from(""))   ;
	let restart               = &*service.action_restart.unwrap_or(String::from(""));
	//

	let start_path = Path::new(start);
	//
	if !start_path.is_file() {
		return 3
	};

	let result_start_path_meta = start_path.metadata();
	//
	match result_start_path_meta {
		//
		Ok(_)  => ()      ,
		Err(_) => return 3,
		//
	}

	let mut proc_cmd = std::process::Command::new(start_path);
	//
	proc_cmd.env_clear();
	//
	let result_process = proc_cmd.spawn();
	//
	match result_process {
		Ok(_)  => ()      ,
		Err(_) => return 4,
	}
	//
	let process = result_process.unwrap();

	// TODO

}


