//! # Hawk client
//!


use com_anlory_hawk::aidl::com::anlory::hawk::IHawkService::IHawkService;
use com_anlory_hawk::binder;
use std::error::Error;
const HAWK_SERVICE_NAME: &str = "hawkservice";

fn main() -> Result<(), Box<dyn Error>> {
    let service = binder::get_interface::<dyn IHawkService>(HAWK_SERVICE_NAME)
        .map_err(|_| " Failed to connect to service")?;

    let  tag = String::from("watchdog");
    let  process_name = String::from("system_server");
    let  pid = 1230;

    let args = [String::from("reason"), String::from("tag")];

     let _ = service.trigger(&tag, &process_name, pid, &args);
     let _ = service.schedule(&tag, &process_name, pid, &args);
    Ok(())
}