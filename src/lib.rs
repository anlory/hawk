//! Implementation of the `IHawkService` AIDL interface.
use com_anlory_hawk::aidl::com::anlory::hawk::IHawkService::IHawkService;
use com_anlory_hawk::binder;


/// The `IHawkService` implementation.
pub struct HawkService;

impl binder::Interface for HawkService {}

impl IHawkService for HawkService {
	fn trigger(&self, tag:&str, process_name:&str, pid:i32, args:&[String]) -> binder::Result<()> {
		println!("HawkService::trigger()");
		println!("tag: {}", tag);
		println!("process_name: {}", process_name);
		println!("pid: {}", pid);
		for arg in args {
			println!("arg: {}", arg);
		}
		Ok(())
	}

	fn schedule(&self, tag:&str, process_name:&str, pid:i32, args:&[String]) -> binder::Result<()> {
		println!("HawkService::schedule()");
        println!("HawkService::trigger()");
        println!("tag: {}", tag);
        println!("process_name: {}", process_name);
        println!("pid: {}", pid);
        for arg in args {
            println!("arg: {}", arg);
        }
		Ok(())
	}
}