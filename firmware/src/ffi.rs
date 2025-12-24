// use crate::rgb::RgbBuffer;

// #[repr(C)]
// #[derive(Debug)]
// pub struct LedChannelInfo {
//     pub framebuffer: *const u8,
//     pub length_in_bytes: u32,
//     pub length_in_pixels: u8,
//     pub channel_number: u8,
// }

// impl LedChannelInfo {
//     pub const fn disabled(channel_number: u8) -> Self {
//         Self {
//             framebuffer: core::ptr::null(),
//             length_in_bytes: 0,
//             length_in_pixels: 0,
//             channel_number,
//         }
//     }

//     pub fn from_buffer<const N: usize>(buffer: &RgbBuffer<N>, channel_number: u8) -> Self {
//         Self {
//             framebuffer: buffer.as_bytes_ptr(),
//             length_in_bytes: (N * 3) as u32,
//             length_in_pixels: N as u8,
//             channel_number,
//         }
//     }
// }

// unsafe extern "C" {
//     pub fn ws2812_init() -> i32;
//     pub fn ws2812_refresh(channels: *const LedChannelInfo, gpio_bank: *mut core::ffi::c_void);

//     // Interrupt handlers
//     pub fn TIM1_UP_TIM10_Handler() -> ();
//     pub fn DMA2_Stream2_Handler() -> ();
// }
