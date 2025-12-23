#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

mod channel;
mod ffi;
mod rgb;

use channel::LedChannel;
use core::cell::UnsafeCell;
use led_effects::{Generator, Pulse, Static};
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
    // DMA2_STREAM2 => Dma2Stream2Handler; // SEE build.rs FOR OVERRIDE
    TIM1_UP_TIM10 => Tim1UpTim10Handler;
});

const LED_COUNT: usize = 400;

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

mod init;
use init::init_clock;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut device_config = embassy_stm32::Config::default();
    init_clock(&mut device_config);

    let peripherals = embassy_stm32::init(device_config);

    unsafe {
        ws2812_init();
    }

    _spawner.spawn(blink(peripherals.PG13.into())).unwrap();
    _spawner.spawn(blink(peripherals.PG14.into())).unwrap();
    _spawner.spawn(led_effects()).unwrap();

    loop {
        Timer::after_secs(1).await;
    } // Keep alive
}

#[embassy_executor::task]
async fn led_effects() {
    info!("Starting LED effects task");

    let mut effect = Pulse {
        start_time: 0,
        duration: 2000,
        position: 30,
        spread_speed: 0.02,
        width: Static(3.0),
        intensity: Static(1.0),
        hue: Static(240.0),
        saturation: Static(1.0),
    };

    loop {
        let now = Instant::now().as_millis();

        if !effect.is_alive(now) {
            effect.start_time = now; // restart
        }

        CHANNEL_0.get().fill(0, 0, 0);
        effect.generate(CHANNEL_0.get().as_mut_slice(), now);

        // Render to LEDs
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

        Timer::after_millis(25).await;
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
