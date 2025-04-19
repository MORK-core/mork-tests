#![no_std]
#![no_main]

use core::sync::atomic::AtomicUsize;

use mork_common::{constants::CNodeSlot, mork_user_log};
use mork_user_lib::mork_ipc::mork_notification_signal;
use mork_user_lib::mork_ipc_buffer::ipc_buffer_init;
use mork_user_lib::mork_task::{create_thread, mork_thread_suspend};
use mork_user_lib::mork_tls::tls_init;

const MAIN_IPC_BUFFER_ADDR: usize = 0x1000_0000;
const MAX_THREAD_NUM: usize = 512;

static COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn child_thread() {
    // mork_user_log!(info, "Hello from child thread!");
    COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    mork_notification_signal(CNodeSlot::CapParentCom as usize).unwrap();
    mork_thread_suspend(CNodeSlot::CapInitThread as usize).unwrap();
}

#[unsafe(no_mangle)]
pub fn main() {
    mork_user_log!(info, "test thread create!");
    if let Err(_) = tls_init() {
        mork_user_log!(error, "Failed to initialize TLS!");
        return;
    }
    mork_user_log!(info, "Successfully initialized TLS!");
    if let Err(_) = ipc_buffer_init(CNodeSlot::CapInitThread as usize, MAIN_IPC_BUFFER_ADDR) {
        mork_user_log!(error, "Failed to initialize IPC context!");
        return;
    }

    for _ in 0..MAX_THREAD_NUM {
        if let Ok(child) = create_thread(child_thread as usize) {
            child.resume().unwrap();
            child.wait().unwrap();
            child.free().unwrap();
        } else {
            mork_user_log!(error, "Failed to create child thread!");
            return;
        }
    }
    assert_eq!(COUNT.load(core::sync::atomic::Ordering::Relaxed), MAX_THREAD_NUM);
}