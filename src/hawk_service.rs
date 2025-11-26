//! Implementation of the `IHawkService` AIDL interface.
use com_anlory_hawk::aidl::com::anlory::hawk::IHawkService::IHawkService;
use com_anlory_hawk::binder;
use log::{debug, info, error};

/// The `IHawkService` implementation.
pub struct HawkService;

impl binder::Interface for HawkService {
    fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> {
        println!("HawkService::dump()");
        info!("HawkService::dump()");
        Ok(())
    }
}

impl IHawkService for HawkService {
	fn trigger(&self, tag:&str, process_name:&str, pid:i32, args:&[String]) -> binder::Result<()> {
		debug!("HawkService::trigger()");
		debug!("tag: {}", tag);
		debug!("process_name: {}", process_name);
		debug!("pid: {}", pid);
		for arg in args {
			debug!("arg: {}", arg);
		}
		Ok(())
	}

	fn schedule(&self, tag:&str, process_name:&str, pid:i32, args:&[String]) -> binder::Result<()> {
		error!("HawkService::schedule()");
        error!("HawkService::trigger()");
        error!("tag: {}", tag);
        error!("process_name: {}", process_name);
        error!("pid: {}", pid);
        for arg in args {
            error!("arg: {}", arg);
        }
		Ok(())
	}
}