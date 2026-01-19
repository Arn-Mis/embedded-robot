use rp2040_pac::{CLOCKS, PLL_SYS, PLL_USB, RESETS, WATCHDOG, XOSC, clocks::{self}};

pub const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub fn init_clocks(
    watchdog: &WATCHDOG,
    xosc: &XOSC,
    clocks: &CLOCKS,
    pll_sys: &PLL_SYS,
    pll_usb: &PLL_USB,
    resets: &RESETS
) {
    
    init_xosc(xosc);
    init_watchdog(watchdog);
    init_pll_sys(clocks, resets, pll_sys);
    init_pll_usb(clocks, resets, pll_usb);
  
    // Set up the reference clock generator

    clocks.clk_ref_div().modify(|_, w| w.int().variant(1));

    clocks.clk_ref_ctrl().modify(|_, w| w.src().xosc_clksrc());

    while clocks.clk_ref_selected().read().bits()
        != 1 << (clocks::clk_ref_ctrl::SRC_A::XOSC_CLKSRC as u8)
    {}

    clocks.clk_ref_div().modify(|_, w| w.int().variant(1));

    // Set up the system clock generator

    clocks.clk_sys_div().modify(|_, w| w.int().variant(1));
   
    clocks.clk_sys_ctrl().modify(|_, w| w.src().clk_ref());

    while clocks.clk_sys_selected().read().bits() != 1 {}

   
    clocks.clk_sys_ctrl()
        .modify(|_, w| w.auxsrc().clksrc_pll_sys());

    clocks.clk_sys_ctrl()
        .modify(|_, w| w.src().clksrc_clk_sys_aux());

    while clocks.clk_sys_selected().read().bits()
        != 1 << (clocks::clk_sys_ctrl::SRC_A::CLKSRC_CLK_SYS_AUX as u8)
    {}

    clocks.clk_sys_div().modify(|_, w| w.int().variant(1));
    
    // Set up the ADC clock

    clocks.clk_adc_div().modify(|_, w| w.int().variant(1));

    clocks.clk_adc_ctrl().modify(|_, w| w.enable().clear_bit());

    clocks.clk_adc_ctrl().modify(|_, w| {
        w.auxsrc()
            .variant(clocks::clk_adc_ctrl::AUXSRC_A::CLKSRC_PLL_USB)
    });

    clocks.clk_adc_ctrl().modify(|_, w| w.enable().set_bit());
    
    clocks.clk_peri_ctrl().modify(|_, w| w.enable().clear_bit());

    clocks.clk_peri_ctrl().modify(|_, w| {
        w.auxsrc()
            .variant(clocks::clk_peri_ctrl::AUXSRC_A::CLK_SYS)
    });

    clocks.clk_peri_ctrl().modify(|_, w| w.enable().set_bit());


}


fn init_xosc(xosc: &XOSC) {
    
    xosc.ctrl().modify(|_, w| w.freq_range()._1_15mhz().enable().enable());

    xosc.startup().write(|w| w.delay().variant(2944)); // Magic number hihi

    xosc.ctrl().modify(|_, w| w.enable().enable());

    while xosc.status().read().stable().bit_is_clear() {}
}

fn init_watchdog(watchdog: &WATCHDOG) {
    watchdog.ctrl().modify(|_, w| w.enable().set_bit());

    watchdog.tick().modify(|_, w| w.cycles().variant((XTAL_FREQ_HZ /1_000_000) as u16));
}


fn init_pll_sys(clocks: &CLOCKS, resets: &RESETS, pll_sys: &PLL_SYS){
    // Default pll_sys config values from the datasheet for a 125MHz system clock
    const REFDIV: u8 = 1;
    const POSTDIV1: u8  = 6;
    const POSTDIV2: u8 = 2;
    const FBDIV: u16 = 125;

    clocks.clk_sys_ctrl().modify(|_, w| w.src().clk_ref());
    while clocks.clk_sys_selected().read().bits() != 1 {}

    resets.reset().modify(|_, w| w.pll_sys().clear_bit());
    while resets.reset_done().read().pll_sys().bit_is_clear() {}

    pll_sys.pwr().reset();

    pll_sys.cs().modify(|_, w | w.refdiv().variant(REFDIV));
    pll_sys.fbdiv_int().modify(|_, w| w.fbdiv_int().variant(FBDIV));
    pll_sys.pwr().modify(|_, w| w
        .pd().clear_bit()
        .vcopd().clear_bit()
    );

    while pll_sys.cs().read().lock().bit_is_clear() {}

    pll_sys.prim().modify(|_, w| w
        .postdiv1().variant(POSTDIV1)
        .postdiv2().variant(POSTDIV2)
    );

    pll_sys.pwr().modify(|_, w| w
        .postdivpd().clear_bit()
    );
}

fn init_pll_usb(clocks: &CLOCKS, resets: &RESETS, pll_usb: &PLL_USB){
    // Default pll_usb config values from the datasheet for a 125MHz system clock
    const REFDIV: u8 = 1;
    const POSTDIV1: u8  = 6;
    const POSTDIV2: u8 = 2;
    const FBDIV: u16 = 125;

    clocks.clk_sys_ctrl().modify(|_, w| w.src().clk_ref());
    while clocks.clk_sys_selected().read().bits() != 1 {}

    resets.reset().modify(|_, w| w.pll_usb().clear_bit());
    while resets.reset_done().read().pll_usb().bit_is_clear() {}

    pll_usb.pwr().reset();

    pll_usb.cs().modify(|_, w | w.refdiv().variant(REFDIV));
    pll_usb.fbdiv_int().modify(|_, w| w.fbdiv_int().variant(FBDIV));
    pll_usb.pwr().modify(|_, w| w
        .pd().clear_bit()
        .vcopd().clear_bit()
    );

    while pll_usb.cs().read().lock().bit_is_clear() {}

    pll_usb.prim().modify(|_, w| w
        .postdiv1().variant(POSTDIV1)
        .postdiv2().variant(POSTDIV2)
    );

    pll_usb.pwr().modify(|_, w| w
        .postdivpd().clear_bit()
    );


}

