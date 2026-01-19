use rp2040_pac::{Peripherals, io_bank0::gpio::gpio_ctrl::FUNCSEL_A, pwm::CH};

use crate::utils::{self, motors::{Motor, MotorControl}};

const TRIG_PIN: u8 = 7;
const ECHO_PIN: u8 = 6;


pub fn assignment_launch(p: &Peripherals) {

    p.IO_BANK0.gpio(6).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::SIO));
    p.IO_BANK0.gpio(7).gpio_ctrl().modify(|_, w| w.funcsel().variant(FUNCSEL_A::SIO));
    
    let ch1: &CH = p.PWM.ch(1);
    let ch2: &CH = p.PWM.ch(2);

    let mot1 = utils::motors::Motor::new(ch1);
    let mot2 = utils::motors::Motor::new(ch2);

    p.SIO.gpio_oe().modify(|_, w| w.gpio_oe().variant(1 << TRIG_PIN | 1 << 25));
    p.SIO.gpio_out_clr().write(|w| w.gpio_out_clr().variant(1 << ECHO_PIN | 1 << TRIG_PIN)); 

    p.IO_BANK0.intr(0).write(|w| w.gpio6_edge_high().variant(true).gpio6_edge_low().variant(true));
    p.IO_BANK0.proc0_inte(0).modify(|_, w| { w.gpio6_edge_high().set_bit().gpio6_edge_low().set_bit() });
    
    start_detection(p, &mot1, &mot2);
    
}

fn start_detection(p: &Peripherals, mot1: &Motor, mot2: &Motor) {
    loop {
        
        let timeout = p.TIMER.timelr().read().bits();
        while p.TIMER.timelr().read().bits().wrapping_sub(timeout) <= 6000 {}

        let begin = p.TIMER.timelr().read().bits();
        p.SIO.gpio_out_set().write(|w| w.gpio_out_set().variant(1 << TRIG_PIN));
        while p.TIMER.timelr().read().bits().wrapping_sub(begin) < 10 {}
        p.SIO.gpio_out_clr().write(|w| w.gpio_out_clr().variant(1 << TRIG_PIN));

        if !unsafe { crate::OBSTACLE } {
            mot1.set_dc(100);
            mot2.set_dc(100);
        } else {
            mot1.set_dc(0);
            mot2.set_dc(0);
        }
         
    }
} 
