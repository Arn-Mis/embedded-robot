#![no_std]
#![no_main]

#![allow(static_mut_refs)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]
mod utils;
mod assignment1;
mod assignment2;
mod assignment3;


use core::panic::PanicInfo;

use cortex_m::{asm, peripheral::NVIC};
use cortex_m_rt::{entry};
use rp2040_pac::{IO_BANK0, TIMER, interrupt};
use rp2040_pac::Interrupt::IO_IRQ_BANK0;
use rp2040_pac::Peripherals;
use rp2040_pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;
use crate::{utils::clocks};

// use rp2040_boot2;
#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
pub static mut ECHO_BEGIN: u32 = 0;
pub static mut OBSTACLE: bool = false;

static SELECTED_MISSION: MISSION = MISSION::VARIABLE_SPEED;


enum MISSION {
    VARIABLE_SPEED,
    OBSTACLE_DETECTION,
    LINE_FOLLOWER
}





// On panic, just loop forever
#[panic_handler]
fn handle_panic(_: &PanicInfo) -> ! {
    let p = unsafe { Peripherals::steal() };

    p.RESETS.reset().modify(|_, w| w.io_bank0().clear_bit().pads_bank0().clear_bit());
    while p.RESETS.reset_done().read().io_bank0().bit_is_clear() {}
    while p.RESETS.reset_done().read().pads_bank0().bit_is_clear() {}

    p.IO_BANK0.gpio(25).gpio_ctrl().write(|w| w.funcsel().variant(FUNCSEL_A::SIO));
    p.SIO.gpio_oe_set().write(|w| w.gpio_oe_set().variant(1 << 25));
    p.SIO.gpio_out_set().write(|w| w.gpio_out_set().variant(1 << 25));

    loop {}
}


pub fn delay(u: u32) {
    asm::delay(u* 6_000_000);
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

    clocks::init_clocks(&peripherals.WATCHDOG, &peripherals.XOSC, &peripherals.CLOCKS, &peripherals.PLL_SYS, &peripherals.PLL_USB, &peripherals.RESETS);

    // Motor gpio initialization to PWM function
    peripherals.IO_BANK0.gpio(18).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    peripherals.IO_BANK0.gpio(19).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    peripherals.IO_BANK0.gpio(20).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));
    peripherals.IO_BANK0.gpio(21).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::PWM));

    peripherals.IO_BANK0.gpio(25).gpio_ctrl().write(| w| w.funcsel().variant(FUNCSEL_A::SIO));
    peripherals.SIO.gpio_oe().modify(|_, w| w.gpio_oe().variant(1 << 25));
  
   
    
    NVIC::unpend(IO_IRQ_BANK0);
    unsafe { NVIC::unmask(IO_IRQ_BANK0) };

    mission_launch(&SELECTED_MISSION, &peripherals);

    loop {
        delay(1);
    }
}

fn mission_launch(mission: &MISSION, peripherals: &Peripherals) {
    match mission {
        MISSION::VARIABLE_SPEED => assignment1::assignment_launch(peripherals),
        MISSION::OBSTACLE_DETECTION => assignment2::assignment_launch(peripherals),
        MISSION::LINE_FOLLOWER => assignment3::assignment_launch(peripherals),
    }
}


#[rp2040_pac::interrupt]
fn IO_IRQ_BANK0() {
    cortex_m::interrupt::free(|_cs| {
        let io_bank0 = unsafe {&*IO_BANK0::ptr()};
        let timer = unsafe {&*TIMER::ptr()};
        let intr = io_bank0.proc0_ints(0).read();

        
        if intr.gpio6_edge_high().bit_is_set() {
            unsafe {ECHO_BEGIN = timer.timelr().read().bits();} 
            io_bank0.intr(0).write(|w| w.gpio6_edge_high().variant(true));
        }

        if intr.gpio6_edge_low().bit_is_set() {
            let d = timer.timelr().read().bits().wrapping_sub(unsafe { ECHO_BEGIN } ) / 58 ;
            if d < 10 {
                unsafe { OBSTACLE = true};
            } else { unsafe {OBSTACLE = false}};
            io_bank0.intr(0).write(|w| w.gpio6_edge_low().variant(true));
        }
    });
}

