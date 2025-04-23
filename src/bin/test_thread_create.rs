#![no_std]
#![no_main]

use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::Relaxed;
use mork_common::{constants::CNodeSlot, mork_user_log};
use mork_user_lib::mork_ipc::mork_notification_signal;
use mork_user_lib::mork_task::mork_task_suspend;
use mork_user_lib::mork_thread::create_thread;
const MAX_THREAD_NUM: usize = 1000;

static COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn child_thread() {
    COUNT.fetch_add(1, Relaxed);
    mork_notification_signal(CNodeSlot::CapParentCom as usize).unwrap();
    mork_task_suspend(CNodeSlot::CapInitThread as usize).unwrap();
}

#[unsafe(no_mangle)]
pub fn main()  {
    mork_user_log!(info, "test thread create!");
    for _ in 0..MAX_THREAD_NUM {
        if let Ok(child) = create_thread(child_thread as usize) {
            child.resume().unwrap();
            child.wait().unwrap();
            child.free().unwrap();
        } else {
            panic!("Failed to create child thread!");
        }
    }
    assert_eq!(COUNT.load(Relaxed), MAX_THREAD_NUM);
}