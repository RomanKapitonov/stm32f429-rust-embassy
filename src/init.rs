pub fn init_clock(device_config: &mut embassy_stm32::Config) {
    use embassy_stm32::rcc::*;
    use embassy_stm32::time::*;
    device_config.enable_debug_during_sleep = true;
    device_config.rcc.hse = Some(Hse {
        freq: mhz(8),
        mode: HseMode::Oscillator,
    });
    device_config.rcc.pll_src = PllSource::HSE;
    device_config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV4,
        mul: PllMul::MUL168,
        divp: Some(PllPDiv::DIV2),
        divq: Some(PllQDiv::DIV7),
        divr: None,
    });
    device_config.rcc.sys = Sysclk::PLL1_P; // 168 MHz
    device_config.rcc.ahb_pre = AHBPrescaler::DIV1; // 168 MHz
    device_config.rcc.apb1_pre = APBPrescaler::DIV4; // 42 MHz, Timer clock 84 MHz
    device_config.rcc.apb2_pre = APBPrescaler::DIV2; // 84 MHz, Timer clock 168 MHz
}
