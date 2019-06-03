#![no_std]
#![no_main]

use cortex_m_rt::entry;

static mut GLOBAL_ARRAY: [u8; 1024] = [0; 1024];

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    unsafe {
        for i in 0..GLOBAL_ARRAY.len() {
            GLOBAL_ARRAY[i] += 1;
        }
    };
    loop {}
}
