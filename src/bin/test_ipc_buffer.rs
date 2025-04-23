#![no_std]
#![no_main]

use mork_common::mork_user_log;

use mork_user_lib::mork_ipc_buffer::{with_ipc_buffer, with_ipc_buffer_mut};

const SET_IPC_USER_DATA: usize = 100;

#[unsafe(no_mangle)]
pub fn main() {
    mork_user_log!(info, "test IPC Buffer!");

    with_ipc_buffer_mut(|buffer| {
        buffer.user_data = SET_IPC_USER_DATA;
    });

    with_ipc_buffer(|buffer| {
        assert_eq!(buffer.user_data, SET_IPC_USER_DATA);
    });
}