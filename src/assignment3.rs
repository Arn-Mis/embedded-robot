use rp2040_pac::{ADC, Peripherals, pwm::CH};

use crate::utils::{self, motors::{Motor, MotorControl}};


#[derive(Copy, Clone, PartialEq, Eq)]
enum STATE {
    STRAIGHT,
    LEFT,
    RIGHT,
}

pub fn assignment_launch(p: &Peripherals) {
    p.RESETS.reset().modify(|_, w| w.io_bank0().clear_bit().pads_bank0().clear_bit().adc().clear_bit());
    while p.RESETS.reset_done().read().io_bank0().bit_is_clear() {}
    while p.RESETS.reset_done().read().pads_bank0().bit_is_clear() {}
    while p.RESETS.reset_done().read().adc().bit_is_clear() {}
    
    
    let ch1: &CH = p.PWM.ch(1);
    let ch2: &CH = p.PWM.ch(2);

    let mot1 = utils::motors::Motor::new(ch1);
    let mot2 = utils::motors::Motor::new(ch2);

    state_machine_loop(&p.ADC, &mot1, &mot2);
}

fn state_machine_loop(adc: &ADC, motor_right: &Motor, motor_left: &Motor) {
    let mut current_state = STATE::STRAIGHT;

    adc.cs().modify(|_, w| w.rrobin().variant(0));
    adc.cs().modify(|_, w| w.en().set_bit());
    while !adc.cs().read().ready().bit_is_set() {}
    

    loop {
        match current_state {
            STATE::STRAIGHT => {
                motor_left.set_dc(67);
                motor_right.set_dc(75);
            },
            STATE::LEFT => {
                motor_left.set_dc(0);
                motor_right.set_dc(80);
            },
            STATE::RIGHT => {
                motor_left.set_dc(82);
                motor_right.set_dc(0);
            },
        }
        let (inner, outer) = read_adc(adc);
        match (inner, outer) {
            (true, true) => {
                // Both sensors on the line -> Turn right
                current_state = STATE::RIGHT;
            },
            (true, false) => {
                // Only the inner sensor is on the line -> Go straight
                current_state = STATE::STRAIGHT;
            },
            (false, true) => {
                // Only the outer sensor is on the line -> Turn left
                current_state = STATE::LEFT;
            },
            (false, false) => {
                // None of the sensors are on the line -> Turn right
                current_state = STATE::RIGHT;
            },
        }


    }

}

fn read_adc(adc: &ADC) -> (bool, bool){

    const IR_INSIDE: u8 = 0;
    const IR_OUTSIDE: u8 = 1;
    
    adc.cs().modify(|_, w| w.ainsel().variant(IR_OUTSIDE));
    adc.cs().modify(|_, w| w.start_once().set_bit());
    while !adc.cs().read().ready().bit_is_set() {}
    let outside = adc.result().read().result().bits() > 750;


    adc.cs().modify(|_, w| w.ainsel().variant(IR_INSIDE));
    adc.cs().modify(|_, w| w.start_once().set_bit());
    while !adc.cs().read().ready().bit_is_set() {}
    let inside = adc.result().read().result().bits() > 750;


    (inside, outside)

}

