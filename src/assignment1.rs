use rp2040_pac::{Peripherals, pwm::CH};

use crate::utils::{self, motors::{Motor, MotorControl}};

pub fn assignment_launch (p: &Peripherals) {
    let ch1: &CH = p.PWM.ch(1);
    let ch2: &CH = p.PWM.ch(2);

    let mot1 = utils::motors::Motor::new(ch1);
    let mot2 = utils::motors::Motor::new(ch2);

    drive(&mot1, &mot2);

}

fn drive(mot1: &Motor, mot2: &Motor) {
    mot1.set_dc(75);
    mot2.set_dc(75);

    super::delay(20);

    mot1.set_dc(100);
    mot2.set_dc(100);

    super::delay(10);

    for i in 0..20 {
        mot1.set_dc(100 - 5 * i);
        mot2.set_dc(100 - 5 * i);
        super::delay(5);
    }

    mot1.set_dc(0);
    mot2.set_dc(0);

}
