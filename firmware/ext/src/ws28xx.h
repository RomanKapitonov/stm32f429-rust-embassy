#pragma once

#include <stdint.h>
#include <stdbool.h>
#include "stm32f4xx.h"

struct led_channel_info {
    const uint8_t *framebuffer;
    uint32_t length_in_bytes;
    uint8_t channel_number;
};
#define WS2812_NUM_CHANNELS 8
extern struct led_channel_info led_channels[WS2812_NUM_CHANNELS];

void ws2812_init(void);
void ws2812_refresh(const struct led_channel_info *channels);
void TIM1_UP_TIM10_Handler(void);
void DMA2_Stream2_Handler(void);
