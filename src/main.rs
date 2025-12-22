#![no_std]
#![no_main]

mod utils;

use core::panic::PanicInfo;

use cortex_m::asm;
use cortex_m_rt::entry;
use rp2040_pac::{Peripherals};

// use rp2040_boot2;
#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

// On panic, just loop forever
#[panic_handler]
fn handle_panic(_: &PanicInfo) -> ! {
    loop {}
}

fn delay() {
    asm::delay(6_000_000);
}
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    // Clears the reset bits of io_bank and pwm, releasing them from the reset state, and waits for
    // resets to finish.
    peripherals.RESETS.reset().modify(|_, w| w.io_bank0().clear_bit().pwm().clear_bit());
    while peripherals.RESETS.reset_done().read().io_bank0().bit_is_clear() {}
    while peripherals.RESETS.reset_done().read().pwm().bit_is_clear() {}
    
    

    loop {
        delay();
    }
}

