use super::ffi::{LedChannelInfo, WS2812_NUM_CHANNELS, led_channels, ws2812_init, ws2812_refresh};
use core::ptr::{addr_of, addr_of_mut};
use led_effects::Pixel;

pub struct LedChannel {
    buffer: &'static mut [Pixel],
    channel_number: u8,
}

impl LedChannel {
    /// SAFETY: The buffer must truly be 'static and not used elsewhere.
    unsafe fn new(channel_number: u8, buffer: &'static mut [Pixel]) -> Self {
        let num_pixels = buffer.len();

        unsafe {
            // Use addr_of_mut! to access the global array safely
            let base_ptr = addr_of_mut!(led_channels);

            // Write the info directly to the specific index
            (*base_ptr)[channel_number as usize] = LedChannelInfo {
                framebuffer: buffer.as_ptr() as *const u8,
                length_in_bytes: (num_pixels as u32) * 3,
                channel_number,
            };
        }

        Self {
            buffer,
            channel_number,
        }
    }

    #[inline]
    pub fn buffer_mut(&mut self) -> &mut [Pixel] {
        self.buffer
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buffer.fill(Pixel::BLACK);
    }
}

// ============================================================================
// DRIVER MANAGER
// ============================================================================

pub struct LedDriver {
    channels: [Option<LedChannel>; WS2812_NUM_CHANNELS],
    initialized: bool,
}

impl LedDriver {
    pub const fn new() -> Self {
        Self {
            channels: [None, None, None, None, None, None, None, None],
            initialized: false,
        }
    }

    pub fn init_hardware(&mut self) {
        if !self.initialized {
            unsafe {
                ws2812_init();
            }
            self.initialized = true;
        }
    }

    /// Now accepts the static buffer as an argument
    pub fn init_channel(&mut self, channel_number: u8, buffer: &'static mut [Pixel]) {
        if (channel_number as usize) < WS2812_NUM_CHANNELS {
            unsafe {
                self.channels[channel_number as usize] =
                    Some(LedChannel::new(channel_number, buffer));
            }
        }
    }

    pub fn channel_mut(&mut self, channel_number: u8) -> Option<&mut LedChannel> {
        self.channels.get_mut(channel_number as usize)?.as_mut()
    }

    pub fn refresh(&self) {
        unsafe {
            // Use addr_of! to get the pointer to the array head
            // without creating a shared reference.
            let channels_ptr = addr_of!(led_channels) as *const LedChannelInfo;
            ws2812_refresh(channels_ptr);
        }
    }
}

// ============================================================================
// GLOBAL STATIC DRIVER
// ============================================================================

use core::cell::RefCell;
use critical_section::Mutex;

static LED_DRIVER: Mutex<RefCell<Option<LedDriver>>> = Mutex::new(RefCell::new(None));

pub fn init_global_driver() {
    critical_section::with(|cs| {
        let mut driver = LedDriver::new();
        driver.init_hardware();
        LED_DRIVER.borrow_ref_mut(cs).replace(driver);
    });
}

pub fn with_driver<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut LedDriver) -> R,
{
    critical_section::with(|cs| {
        LED_DRIVER
            .borrow_ref_mut(cs)
            .as_mut()
            .map(|driver| f(driver))
    })
}
