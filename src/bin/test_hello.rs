#![no_std]
#![no_main]

use mork_common::mork_user_log;

#[unsafe(no_mangle)]
pub fn main() {
    mork_user_log!(info, "test hello!");
    mork_user_lib::dummy_function();
}