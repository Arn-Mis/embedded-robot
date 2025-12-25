use rp2040_pac::{generic::FieldWriter, pwm::{self, CH, ch::cc::CC_SPEC}};


// Create a direction trait and implement it for the W (A_W) struct 
pub struct Motor {
    channel: &'static CH,
    direction: Direction
}

pub trait MotorControl {
    fn set_dc(&self, percent: usize);
    fn direction(&mut self, dir: Direction);
}

pub enum Direction {
    FORTH,
    REVERSE
}

impl MotorControl for Motor {
    fn direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    fn set_dc(&self, percent: usize) {
        match self.direction {
        Direction::FORTH => self.channel
                                .cc()
                                .modify(|_, w| unsafe { w.a().bits(percent as u16) }),
        Direction::REVERSE => self.channel
                                .cc()
                                .modify(|_, w| unsafe { w.b().bits(percent as u16) })
        }
    }
}

impl Motor {
    pub fn new(channel: &'static CH) -> Self {
        channel.cc().reset();
        channel.div().reset();
        channel.csr().reset();
        channel.top().reset();

        channel.div().modify(|_, w| w
            .int().variant(100)
            .frac().variant(0));
        
        channel.top().modify(|_, w| w
            .top().variant(100));
        
        channel.csr().modify(|_, w| w
                .a_inv().set_bit()
                .b_inv().set_bit()
                .en().set_bit());
        
        Motor {channel, direction: Direction::FORTH }
    }
}
