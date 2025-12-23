#![no_std]
#![no_main]

mod utils;

use core::panic::PanicInfo;

use cortex_m::{asm};
use cortex_m_rt::entry;
use rp2040_pac::{Peripherals, pwm::CH};
use rp2040_pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;
use crate::utils::motors::MotorControl;

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

    peripherals.IO_BANK0.gpio(18).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    peripherals.IO_BANK0.gpio(19).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    peripherals.IO_BANK0.gpio(20).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    peripherals.IO_BANK0.gpio(21).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    
    assignment_1(peripherals);

    loop {
        delay();
    }
}

fn assignment_1(peripherals: Peripherals) {
    let ch1: &CH = peripherals.PWM.ch(1);
    let ch2: &CH = peripherals.PWM.ch(2);

    let mot1 = utils::motors::Motor::new(ch1);
    let mot2 = utils::motors::Motor::new(ch2);

    mot1.set_dc(60);
    mot2.set_dc(60);

    delay();
    delay();
    delay();
    delay();
    delay();

    mot1.set_dc(100);
    mot2.set_dc(100);
    
    delay();
    delay();
    delay();
    delay();
    delay();
    
    for i in 0..100 {
        mot1.set_dc(100 - i);
        mot2.set_dc(100 - i);
    }

}
