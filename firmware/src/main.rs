#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

mod driver;
mod init;

use embassy_stm32::Peri;
use embassy_stm32::bind_interrupts;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::interrupt::typelevel::Handler;
use embassy_stm32::interrupt::typelevel::{DMA2_STREAM2, TIM1_UP_TIM10};

pub struct Tim1UpTim10Handler;
impl Handler<TIM1_UP_TIM10> for Tim1UpTim10Handler {
    unsafe fn on_interrupt() {
        unsafe {
            driver::ffi::TIM1_UP_TIM10_Handler();
        }
    }
}

pub struct Dma2Stream2Handler;
impl Handler<DMA2_STREAM2> for Dma2Stream2Handler {
    unsafe fn on_interrupt() {
        unsafe {
            driver::ffi::DMA2_Stream2_Handler();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DMA2_STREAM2_OVERRIDE() {
    unsafe {
        Dma2Stream2Handler::on_interrupt();
    }
}

bind_interrupts!(struct Irqs {
    TIM1_UP_TIM10 => Tim1UpTim10Handler;
});

use init::init_clock;

use led_effects::Pixel;
const NUM_LEDS: usize = 60;
static mut CHANNEL_0_RAM: [Pixel; NUM_LEDS] = [Pixel::BLACK; NUM_LEDS];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut device_config = embassy_stm32::Config::default();
    init_clock(&mut device_config);

    let peripherals = embassy_stm32::init(device_config);

    // Initialize the global LED driver
    driver::init_global_driver();

    // Setup channel 0
    driver::with_driver(|driver| unsafe {
        use core::ptr::addr_of_mut;

        let buf_ptr = addr_of_mut!(CHANNEL_0_RAM);
        driver.init_channel(0, &mut *buf_ptr);
        // driver.init_channel(0, &mut CHANNEL_0_RAM);
    });

    // _spawner.spawn(blink(peripherals.PG13.into())).unwrap();
    _spawner.spawn(blink(peripherals.PG14.into())).unwrap();
    _spawner
        .spawn(led_effects(peripherals.PG13.into()))
        .unwrap();

    loop {
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn led_effects(debug_pin: Peri<'static, AnyPin>) {
    // Get start time
    let mut pg13_debug: Output<'_> = Output::new(debug_pin, Level::High, Speed::Low);
    let start_time = Instant::now().as_millis();
    let mut effect_start_time = Instant::now(); // Track the start of the current loop
    let effect_duration_ms: u64 = 5000; // Must match your EffectBuilder duration

    // Create the effect ONCE before the loop
    use led_effects::Chase;
    use led_effects::EffectBuilder;
    use led_effects::Generator;
    use led_effects::{Blur, Trail};
    use led_effects::{DynamicParam, Fade, RotatingHue, StaticParam, VelocityIntegral};

    let mut effect = EffectBuilder::new(Chase {
        start_time: 0,
        duration: 5000,

        position: DynamicParam {
            envelope: VelocityIntegral {
                start_time: 0,
                velocity_envelope: Fade {
                    start_time: 0,
                    duration: 5000,
                    inverted: true,
                },
                initial_position: 0.0,
            },
            min: 0.0,
            max: NUM_LEDS as f32,
        },

        width: DynamicParam {
            envelope: Fade {
                start_time: 0,
                duration: 5000,
                inverted: false,
            },
            min: 2.0,
            max: 8.0,
        },

        intensity: DynamicParam {
            envelope: Fade {
                start_time: 0,
                duration: 5000,
                inverted: true,
            },
            min: 0.0,
            max: 1.0,
        },

        hue: RotatingHue {
            start_time: 0,
            degrees_per_ms: 0.05,
        },

        saturation: StaticParam(1.0),
    })
    .with_modifier(Trail {
        decay_rate: DynamicParam {
            envelope: Fade {
                start_time: 0,
                duration: 5000,
                inverted: false,
            },
            min: 0.80,
            max: 0.95,
        },
    })
    .with_modifier(Blur {
        strength: StaticParam(0.2),
    })
    .build();

    info!("LED effects task started");

    loop {
        let now_instant = Instant::now();
        let mut elapsed = now_instant.duration_since(effect_start_time).as_millis();

        if elapsed >= effect_duration_ms {
            effect_start_time = now_instant;
            elapsed = 0;
            info!("Effect restarting...");
        }

        driver::with_driver(|driver| {
            if let Some(channel) = driver.channel_mut(0) {
                channel.clear();
                // Pass the 'elapsed' which now resets to 0 every 5 seconds
                pg13_debug.set_high();
                effect.generate(channel.buffer_mut(), elapsed);
                pg13_debug.set_low();
            }
        });

        driver::with_driver(|driver| {
            driver.refresh();
        });

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
