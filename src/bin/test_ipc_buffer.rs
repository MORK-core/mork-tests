#![no_std]
#![no_main]

use mork_common::mork_user_log;

const MAIN_IPC_BUFFER_ADDR: usize = 0x1000_0000;

use mork_common::constants::CNodeSlot;
use mork_user_lib::mork_tls::tls_init;
use mork_user_lib::mork_ipc_buffer::{ipc_buffer_init, with_ipc_buffer, with_ipc_buffer_mut};

const SET_IPC_USER_DATA: usize = 100;

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

    with_ipc_buffer_mut(|buffer| {
        buffer.user_data = SET_IPC_USER_DATA;
    });

    with_ipc_buffer(|buffer| {
        assert_eq!(buffer.user_data, SET_IPC_USER_DATA);
    });
}