#![no_std]
#![no_main]

use mork_common::constants::CNodeSlot;
use mork_common::mork_user_log;
use mork_user_lib::mork_task::mork_thread_suspend;

#[unsafe(no_mangle)]
pub fn main() {
    mork_user_lib::init();
    mork_user_log!(info, "mork test hello1!");
    mork_thread_suspend(CNodeSlot::CapInitThread as usize).unwrap();
}