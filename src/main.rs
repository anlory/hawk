//! # Hawk Logger
//!
//! 这是一个用于 Android 的 Rust 日志记录器 crate。
//! 提供高效的日志记录功能。

use hawkservice::HawkService;
use com_anlory_hawk::aidl::com::anlory::hawk::IHawkService::BnHawkService;
use com_anlory_hawk::binder;

const HAWK_SERVICE_NAME: &str = "hawkservice";
fn main() {
    println!("Hello, world!");

    let hawk = HawkService;
    let hawk_binder = BnHawkService::new_binder(hawk, binder::BinderFeatures::default());

    binder::add_service(HAWK_SERVICE_NAME, hawk_binder.as_binder()).expect("Failed to add service?");

    binder::ProcessState::join_thread_pool();
}