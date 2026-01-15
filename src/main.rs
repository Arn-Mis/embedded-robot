#![no_std]
#![no_main]

mod utils;

use core::panic::PanicInfo;

use cortex_m::{asm, peripheral::NVIC};
use cortex_m_rt::{entry};
use rp2040_pac::{IO_BANK0, SIO, TIMER, interrupt};
use rp2040_pac::Interrupt::IO_IRQ_BANK0;
use rp2040_pac::{Peripherals, pwm::CH};
use rp2040_pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;
use crate::utils::motors::MotorControl;

// use rp2040_boot2;
#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
pub static mut ECHO_BEGIN: u32 = 0;


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

    peripherals.RESETS.reset().modify(|_, w| {
        w.io_bank0().clear_bit()
            .pads_bank0().clear_bit()
            .timer().clear_bit()
            .pwm().clear_bit()
    });
    while peripherals.RESETS.reset_done().read().io_bank0().bit_is_clear() {}
    while peripherals.RESETS.reset_done().read().pads_bank0().bit_is_clear() {}
    while peripherals.RESETS.reset_done().read().timer().bit_is_clear() {}
    while peripherals.RESETS.reset_done().read().pwm().bit_is_clear() {}

    
    // Motor gpio initialization to PWM function
    peripherals.IO_BANK0.gpio(18).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    peripherals.IO_BANK0.gpio(19).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    peripherals.IO_BANK0.gpio(20).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    peripherals.IO_BANK0.gpio(21).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));

    peripherals.IO_BANK0.gpio(25).gpio_ctrl().write(| w| w.funcsel().variant(FUNCSEL_A::SIO));
    peripherals.SIO.gpio_oe().modify(|_, w| w.gpio_oe().variant(1 << 25));
    
    NVIC::unpend(IO_IRQ_BANK0);
    unsafe { NVIC::unmask(IO_IRQ_BANK0) };
    
    // assignment_1(peripherals);
    assignment2(peripherals);

    loop {
        delay();
    }
}

fn assignment_1(peripherals: Peripherals) {
    let ch1: &CH = peripherals.PWM.ch(1);
    let ch2: &CH = peripherals.PWM.ch(2);

    let mot1 = utils::motors::Motor::new(ch1);
    let mut mot2 = utils::motors::Motor::new(ch2);

    mot2.direction(utils::motors::Direction::REVERSE);

    mot1.set_dc(60);
    mot2.set_dc(60);

    delay();

    mot1.set_dc(100);
    mot2.set_dc(100);
    
    delay();
    
    for i in 0..4 {
        mot1.set_dc(100 - 25 *i);
        mot2.set_dc(100 - 25*i);
        delay();
    }

    mot1.set_dc(0);
    mot2.set_dc(0);

}

fn assignment2(p: Peripherals) {
    p.IO_BANK0.gpio(6).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::SIO));
    p.IO_BANK0.gpio(7).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::SIO));
    
    let trig_pin = 7;
    let echo_pin = 6;

    p.SIO.gpio_oe().modify(|_, w| w.gpio_oe().variant(1 << trig_pin | 1 << 25));
    p.SIO.gpio_out_clr().write(|w| w.gpio_out_clr().variant(1 << echo_pin | 1 << trig_pin)); 

    p.IO_BANK0.intr(0).write(|w| w.gpio6_edge_high().variant(true).gpio6_edge_low().variant(true));
    p.IO_BANK0.proc0_inte(0).modify(|_, w| { w.gpio6_edge_high().set_bit().gpio6_edge_low().set_bit() });

    loop {
        
        // Recommended measurement cycle is over 60 ms to prevent something or other.
        let timeout = p.TIMER.timelr().read().bits();
        while p.TIMER.timelr().read().bits().wrapping_sub(timeout) <= 60000 {}

        let begin = p.TIMER.timelr().read().bits();
        p.SIO.gpio_out_set().write(|w| w.gpio_out_set().variant(1 << trig_pin));
        while p.TIMER.timelr().read().bits().wrapping_sub(begin) < 10 {}
        p.SIO.gpio_out_clr().write(|w| w.gpio_out_clr().variant(1 << trig_pin));
        
    }
}

#[rp2040_pac::interrupt]
fn IO_IRQ_BANK0() {
    cortex_m::interrupt::free(|_cs| {
        let io_bank0 = unsafe {&*IO_BANK0::ptr()};
        let timer = unsafe {&*TIMER::ptr()};
        let sio = unsafe {&*SIO::ptr()};
        let intr = io_bank0.intr(0).read();

        
        if intr.gpio6_edge_high().bit_is_set() {
            unsafe {ECHO_BEGIN = timer.timelr().read().bits();} 
            io_bank0.intr(0).write(|w| w.gpio6_edge_high().variant(true));
        }

        if intr.gpio6_edge_low().bit_is_set() {
            if timer.timelr().read().bits().wrapping_sub(unsafe { ECHO_BEGIN } ) / 58 < 10 {
                sio.gpio_out_set().write(|w| w.gpio_out_set().variant(1 << 25));
            }
            io_bank0.intr(0).write(|w| w.gpio6_edge_low().variant(true));
        }
    });
}
