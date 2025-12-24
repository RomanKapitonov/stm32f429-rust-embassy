#[repr(C)]
pub struct LedChannelInfo {
    pub framebuffer: *const u8,
    pub length_in_bytes: u32,
    pub channel_number: u8,
}

unsafe extern "C" {
    pub static mut led_channels: [LedChannelInfo; 8];
    pub fn ws2812_init();
    pub fn ws2812_refresh(channels: *const LedChannelInfo);
    // Interrupt handlers
    pub fn TIM1_UP_TIM10_Handler() -> ();
    pub fn DMA2_Stream2_Handler() -> ();
}

pub const WS2812_NUM_CHANNELS: usize = 8;
