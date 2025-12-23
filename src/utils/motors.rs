use rp2040_pac::pwm::CH;

pub struct Motor<'a> {
    channel: &'a CH,
    
}

pub trait MotorControl<'a> {
    fn set_dc(&self, percent: usize);
}

impl<'a> MotorControl<'a> for Motor<'a> {
    fn set_dc(&self, percent: usize) {
        self.channel
            .cc()
            .modify(|_, w| unsafe { w.a().bits(percent as u16) });
    }
}

impl<'a> Motor<'a> {
    pub fn new(channel: &'a CH) -> Self {
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
        
        Motor {channel}
    }
}
