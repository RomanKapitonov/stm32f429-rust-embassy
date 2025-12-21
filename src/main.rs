#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

mod channel;
mod ffi;
mod rgb;

use channel::LedChannel;
use core::cell::UnsafeCell;
use embassy_stm32::Peri;
use embassy_stm32::bind_interrupts;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::interrupt::typelevel::Handler;
use embassy_stm32::interrupt::typelevel::{DMA2_STREAM2, TIM1_UP_TIM10};
use ffi::{LedChannelInfo, ws2812_init, ws2812_refresh};

pub struct Tim1UpTim10Handler;
impl Handler<TIM1_UP_TIM10> for Tim1UpTim10Handler {
    unsafe fn on_interrupt() {
        unsafe {
            ffi::TIM1_UP_TIM10_Handler();
        }
    }
}

pub struct Dma2Stream2Handler;
impl Handler<DMA2_STREAM2> for Dma2Stream2Handler {
    unsafe fn on_interrupt() {
        unsafe {
            ffi::DMA2_Stream2_Handler();
        }
    }
}

// This symbol will be used on link phase to resolve the conflict
// see https://github.com/embassy-rs/embassy/issues/4597
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DMA2_STREAM2_OVERRIDE() {
    unsafe {
        Dma2Stream2Handler::on_interrupt();
    }
}

bind_interrupts!(struct Irqs {
    // DMA2_STREAM2 => Dma2Stream2Handler;
    TIM1_UP_TIM10 => Tim1UpTim10Handler;
});

const LED_COUNT: usize = 100;

// Wrapper for safe static mutable access
struct StaticChannel<const N: usize>(UnsafeCell<LedChannel<N>>);

unsafe impl<const N: usize> Sync for StaticChannel<N> {}

impl<const N: usize> StaticChannel<N> {
    const fn new(channel_number: u8) -> Self {
        Self(UnsafeCell::new(LedChannel::new(channel_number)))
    }

    fn get(&self) -> &mut LedChannel<N> {
        unsafe { &mut *self.0.get() }
    }
}

// Static buffers for channels
static CHANNEL_0: StaticChannel<LED_COUNT> = StaticChannel::new(0);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut device_config = embassy_stm32::Config::default();
    {
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
    let peripherals = embassy_stm32::init(device_config);

    unsafe {
        ws2812_init();
        info!("WS2812 initialized");
    }

    _spawner.spawn(blink(peripherals.PG13.into())).unwrap();
    _spawner.spawn(blink(peripherals.PG14.into())).unwrap();

    let mut counter: u8 = 0;
    let mut ticks: u32 = 0;

    loop {
        if ticks % 100 == 0 {
            match counter % 2 {
                0 => CHANNEL_0.get().fill(0, 0, 255),
                1 => CHANNEL_0.get().fill(255, 0, 0),
                _ => {}
            }
            counter = counter.wrapping_add(1);
        }

        // Render every 10ms
        let channels = [
            CHANNEL_0.get().channel_info(),
            LedChannelInfo::disabled(1),
            LedChannelInfo::disabled(2),
            LedChannelInfo::disabled(3),
            LedChannelInfo::disabled(4),
            LedChannelInfo::disabled(5),
            LedChannelInfo::disabled(6),
            LedChannelInfo::disabled(7),
        ];

        unsafe {
            ws2812_refresh(channels.as_ptr(), core::ptr::null_mut());
        }

        ticks = ticks.wrapping_add(1);
        Timer::after_millis(10).await;
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn blink(pin: Peri<'static, AnyPin>) {
    info!("Starting blink task");
    let mut led: Output<'_> = Output::new(pin, Level::High, Speed::Low);

    loop {
        led.set_high();
        Timer::after_millis(500).await;

        led.set_low();
        Timer::after_millis(500).await;
    }
}
