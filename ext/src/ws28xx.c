#include "ws28xx.h"

/* Timer runs at 168MHz on STM32F429 */
#define TIMER1_FREQ_HZ          168000000

/* WS2812 timing: 1.25µs period, 0.4µs (T0H), 0.8µs (T1H) */
#define WS2812_TIMER_PERIOD        (((TIMER1_FREQ_HZ / 1000) * 125) / 100000)
#define WS2812_TIMER_PWM_CH1_TIME  (((TIMER1_FREQ_HZ / 1000) *  40) / 100000)
#define WS2812_TIMER_PWM_CH2_TIME  (((TIMER1_FREQ_HZ / 1000) *  80) / 100000)
#define RESET_TIMER_PERIOD         (WS2812_TIMER_PERIOD * 240)  // ~300µs reset

#define WS2812_NUM_CHANNELS     8
#define WS212_ALL_CHANNELS_SAME_LENGTH  0

/* GPIO assignments (all on GPIOB) */
#define WS2812_CH0_GPIO      0
#define WS2812_CH1_GPIO      1
#define WS2812_CH2_GPIO      2
#define WS2812_CH3_GPIO      3
#define WS2812_CH4_GPIO      4
#define WS2812_CH5_GPIO      5
#define WS2812_CH6_GPIO      6
#define WS2812_CH7_GPIO      7

////////////////////////////////
// DEBUG MACROS
#define DBG_PB8_HIGH()    (GPIOB->BSRR = (1 << 8))
#define DBG_PB8_LOW()     (GPIOB->BSRR = (1 << (8 + 16)))

#define DBG_PB9_HIGH()    (GPIOB->BSRR = (1 << 9))
#define DBG_PB9_LOW()     (GPIOB->BSRR = (1 << (9 + 16)))

#define DBG_PB9_PULSE()   do { \
    GPIOB->BSRR = (1 << 9); \
    __asm__ volatile ("nop"); \
    GPIOB->BSRR = (1 << (9 + 16)); \
} while(0)

#define DBG_PB9_TOGGLE()  (GPIOB->ODR ^= (1 << 9))

////////////////////////////////

#define DMA_BUFFER_SIZE         16
#define DMA_BUFFER_FILL_SIZE    ((DMA_BUFFER_SIZE) / 2)

static uint16_t ws2812_gpio_set_bits = 0;
static uint16_t dma_buffer[DMA_BUFFER_SIZE];

/* DEBUG COUNTERS - Check these in your debugger/Rust code */
volatile uint32_t debug_dma_irq_count = 0;
volatile uint32_t debug_tim_irq_count = 0;

int hal_dma_error_flag = 0;
int framebuffer_position = 0;
int channels_max_length = 0;

/* Global variables */
struct led_channel_info led_channels[WS2812_NUM_CHANNELS];

const static uint8_t ws2812_channel_gpio_map[WS2812_NUM_CHANNELS] = {
    WS2812_CH0_GPIO,
    WS2812_CH1_GPIO,
    WS2812_CH2_GPIO,
    WS2812_CH3_GPIO,
    WS2812_CH4_GPIO,
    WS2812_CH5_GPIO,
    WS2812_CH6_GPIO,
    WS2812_CH7_GPIO,
};

static void ws2812_gpio_init(void)
{
    RCC->AHB1ENR |= RCC_AHB1ENR_GPIOBEN;    // Enable GPIOB clock
    volatile uint32_t dummy = RCC->AHB1ENR; // Delay after enabling clock
    (void)dummy;
    
    // Configure PB0-PB7 as outputs, very high speed, push-pull
    // PB8 is bits 16-17 in MODER
    // PB9 (IRQ Pulse)
    for (int i = 0; i <= 9; i++) {
        GPIOB->MODER &= ~(3 << (i * 2));  // Clear mode bits
        GPIOB->MODER |= (1 << (i * 2));   // Output
        GPIOB->OSPEEDR |= (3 << (i * 2)); // Very High Speed
    }
}

static void ws2812_timer1_init(void)
{
    RCC->APB2ENR |= RCC_APB2ENR_TIM1EN; // Enable TIM1 clock
    
    // Reset TIM1
    TIM1->CR1 = 0; // Disable timer, reset control register
    TIM1->CR2 = 0; // Reset output compare settings
    TIM1->SMCR = 0; // Disable slave mode
    TIM1->DIER = 0; // Disable all interrupts and DMA requests
    TIM1->SR = 0; // Clear status register
    
    // Set prescaler and period
    TIM1->PSC = 0; // No prescaler
    TIM1->ARR = WS2812_TIMER_PERIOD; // Set period auto reload register
    
    // Configure CH1 in PWM mode 1
    TIM1->CCMR1 &= ~(TIM_CCMR1_OC1M | TIM_CCMR1_CC1S); // Clear Output Compare 1 Mode and Channel 1 Selection bits
    TIM1->CCMR1 |= (6 << TIM_CCMR1_OC1M_Pos);  // Set OC1M=110b (PWM mode 1: output high when CNT < CCR1)
    TIM1->CCR1 = WS2812_TIMER_PWM_CH1_TIME; // Set PWM pulse width/duty cycle for channel 1
    TIM1->CCER |= TIM_CCER_CC1E;  // Enable channel 1 output (activate output on pin)
    
    // Configure CH2 in PWM mode 1
    TIM1->CCMR1 &= ~(TIM_CCMR1_OC2M | TIM_CCMR1_CC2S); // Clear Output Compare 2 Mode and Channel 2 Selection bits
    TIM1->CCMR1 |= (6 << TIM_CCMR1_OC2M_Pos);  // Set OC2M=110b (PWM mode 1: output high when CNT < CCR2)
    TIM1->CCR2 = WS2812_TIMER_PWM_CH2_TIME; // Set PWM pulse width/duty cycle for channel 2
    TIM1->CCER |= TIM_CCER_CC2E;  // Enable channel 2 output (activate output on pin)
    
    TIM1->BDTR |= TIM_BDTR_MOE;  // Main output enable
    NVIC_ClearPendingIRQ(TIM1_UP_TIM10_IRQn); // Clear pending interrupt
    // Enable TIM1 update interrupt
    NVIC_SetPriority(TIM1_UP_TIM10_IRQn, 0); // Highest priority
    NVIC_EnableIRQ(TIM1_UP_TIM10_IRQn); // Enable interrupt in NVIC
}

static void ws2812_dma_init(void)
{
    RCC->AHB1ENR |= RCC_AHB1ENR_DMA2EN; // Enable DMA2 clock
    
    // Configure DMA2 Stream5 (TIM1_UP) - sets all bits high
    DMA2_Stream5->CR = 0; // Clear control register
    while (DMA2_Stream5->CR & DMA_SxCR_EN);  // Wait until disabled
    
    DMA2_Stream5->PAR = (uint32_t)&GPIOB->BSRR;           // Set peripheral address, lower 16 bits
    DMA2_Stream5->M0AR = (uint32_t)&ws2812_gpio_set_bits; // Set memory address
    DMA2_Stream5->NDTR = DMA_BUFFER_SIZE;                 // Number of data items to transfer
    DMA2_Stream5->CR = (6 << DMA_SxCR_CHSEL_Pos) |        // Channel 6
                       DMA_SxCR_PL_1 | DMA_SxCR_PL_0 |    // Very high priority
                       DMA_SxCR_MSIZE_0 |                 // 16-bit memory
                       DMA_SxCR_PSIZE_0 |                 // 16-bit peripheral
                       DMA_SxCR_CIRC |                    // Circular mode
                       DMA_SxCR_DIR_0;                    // Memory to peripheral
    
    // Configure DMA2 Stream1 (TIM1_CH1) - sets bits based on data
    DMA2_Stream1->CR = 0; // Clear control register
    while (DMA2_Stream1->CR & DMA_SxCR_EN); // Wait until disabled
    
    DMA2_Stream1->PAR = (uint32_t)&GPIOB->BSRR + 2;    // Upper 16 bits (reset)
    DMA2_Stream1->M0AR = (uint32_t)dma_buffer;         // Set memory address
    DMA2_Stream1->NDTR = DMA_BUFFER_SIZE;              // Number of data items to transfer
    DMA2_Stream1->CR = (6 << DMA_SxCR_CHSEL_Pos) |     // Channel 6
                       DMA_SxCR_PL_1 | DMA_SxCR_PL_0 | // Very high priority
                       DMA_SxCR_MSIZE_0 |              // 16-bit memory
                       DMA_SxCR_PSIZE_0 |              // 16-bit peripheral
                       DMA_SxCR_MINC |                 // Memory increment
                       DMA_SxCR_CIRC |                 // Circular mode
                       DMA_SxCR_DIR_0;                 // Memory to peripheral
    
    // Configure DMA2 Stream2 (TIM1_CH2) - clears remaining bits
    DMA2_Stream2->CR = 0; // Clear control register
    while (DMA2_Stream2->CR & DMA_SxCR_EN); // Wait until disabled
    
    DMA2_Stream2->PAR = (uint32_t)&GPIOB->BSRR + 2;       // Upper 16 bits (reset)
    DMA2_Stream2->M0AR = (uint32_t)&ws2812_gpio_set_bits; // Set memory address
    DMA2_Stream2->NDTR = DMA_BUFFER_SIZE;                 // Number of data items to transfer
    DMA2_Stream2->CR = (6 << DMA_SxCR_CHSEL_Pos) |        // Channel 6
                       DMA_SxCR_PL_1 | DMA_SxCR_PL_0 |    // Very high priority
                       DMA_SxCR_MSIZE_0 |                 // 16-bit memory
                       DMA_SxCR_PSIZE_0 |                 // 16-bit peripheral
                       DMA_SxCR_CIRC |                    // Circular mode
                       DMA_SxCR_DIR_0 |                   // Memory to peripheral
                       DMA_SxCR_TCIE |                    // Transfer complete interrupt
                       DMA_SxCR_HTIE;                     // Half transfer interrupt
    
    // Enable DMA2 Stream2 interrupt
    NVIC_SetPriority(DMA2_Stream2_IRQn, 0); // Highest priority
    NVIC_EnableIRQ(DMA2_Stream2_IRQn);      // Enable interrupt in NVIC
}

// Extract bit 0-7 (MSB) from ch_val, insert into cur0-cur7 at gpio_num position
// UBFX: Unsigned Bit Field Extract - extract 1 bit at position 7
// BFI: Bit Field Insert - insert r0 into cur0 at gpio_num
// and so on for bits 6-0
// essentially unpacks each bit of ch_val into the correct position in cur0-cur7
#define UNPACK_CHANNEL(gpio_num)                    \
    asm volatile (                                  \
    "ubfx   r0, %[ch_val], #7, #1 \n"               \
    "bfi    %[cur0], r0,   #" #gpio_num ", #1  \n"  \
    "ubfx   r0, %[ch_val], #6, #1 \n"               \
    "bfi    %[cur1], r0,   #" #gpio_num ", #1  \n"  \
    "ubfx   r0, %[ch_val], #5, #1 \n"               \
    "bfi    %[cur2], r0,   #" #gpio_num ", #1  \n"  \
    "ubfx   r0, %[ch_val], #4, #1 \n"               \
    "bfi    %[cur3], r0,   #" #gpio_num ", #1  \n"  \
    "ubfx   r0, %[ch_val], #3, #1 \n"               \
    "bfi    %[cur4], r0,   #" #gpio_num ", #1  \n"  \
    "ubfx   r0, %[ch_val], #2, #1 \n"               \
    "bfi    %[cur5], r0,   #" #gpio_num ", #1  \n"  \
    "ubfx   r0, %[ch_val], #1, #1 \n"               \
    "bfi    %[cur6], r0,   #" #gpio_num ", #1  \n"  \
    "ubfx   r0, %[ch_val], #0, #1 \n"               \
    "bfi    %[cur7], r0,   #" #gpio_num ", #1  \n"  \
    : [cur0]"+r" (cur0),                            \
      [cur1]"+r" (cur1),                            \
      [cur2]"+r" (cur2),                            \
      [cur3]"+r" (cur3),                            \
      [cur4]"+r" (cur4),                            \
      [cur5]"+r" (cur5),                            \
      [cur6]"+r" (cur6),                            \
      [cur7]"+r" (cur7)                             \
    : [ch_val]"r" (ch_val)                          \
    : "r0", "cc");

#define HANDLE_CHANNEL(ch_num, gpio_num)                            \
    if (ch_num < WS2812_NUM_CHANNELS) {                             \
        ch_val = get_channel_byte(channels + ch_num, pos, ch_num);  \
        UNPACK_CHANNEL(gpio_num);                                   \
    }

static inline uint8_t get_channel_byte(const struct led_channel_info *channel, int pos, int channel_number)
{
    if (WS212_ALL_CHANNELS_SAME_LENGTH || (pos < channel->length_in_bytes)) {
        return channel->framebuffer[pos] ^ 0xff;
    }
    return 0xff;
}

static void fill_dma_buffer(uint16_t *dest, int pos, const struct led_channel_info *channels)
{
    register uint16_t cur0 = 0, cur1 = 0, cur2 = 0, cur3 = 0, cur4 = 0, cur5 = 0, cur6 = 0, cur7 = 0;
    uint8_t ch_val;

    HANDLE_CHANNEL(0, WS2812_CH0_GPIO);
    HANDLE_CHANNEL(1, WS2812_CH1_GPIO);
    HANDLE_CHANNEL(2, WS2812_CH2_GPIO);
    HANDLE_CHANNEL(3, WS2812_CH3_GPIO);
    HANDLE_CHANNEL(4, WS2812_CH4_GPIO);
    HANDLE_CHANNEL(5, WS2812_CH5_GPIO);
    HANDLE_CHANNEL(6, WS2812_CH6_GPIO);
    HANDLE_CHANNEL(7, WS2812_CH7_GPIO);

    dest[0] = cur0;
    dest[1] = cur1;
    dest[2] = cur2;
    dest[3] = cur3;
    dest[4] = cur4;
    dest[5] = cur5;
    dest[6] = cur6;
    dest[7] = cur7;
}

/* IRQ Handlers */
void DMA2_Stream2_Handler(void)
{
    // Check half transfer
    if (DMA2->LISR & DMA_LISR_HTIF2) {   // Half transfer complete
        DMA2->LIFCR = DMA_LIFCR_CHTIF2;  // Clear half transfer complete flag
        
        if (framebuffer_position >= channels_max_length) {
            GPIOB->BSRR = (uint32_t)ws2812_gpio_set_bits << 16; // Write to BR[31:16] to reset pins
        } else {
            for (int i = 0; i < DMA_BUFFER_FILL_SIZE; i += 8) {
                fill_dma_buffer(dma_buffer + i, framebuffer_position, led_channels);
                framebuffer_position++;
            }
        }
    }
    
    // Check transfer complete
    if (DMA2->LISR & DMA_LISR_TCIF2) {  // Transfer complete
        DMA2->LIFCR = DMA_LIFCR_CTCIF2; // Clear transfer complete flag
        
        if (framebuffer_position >= channels_max_length) {
            // Stop timer and disable DMA requests
            TIM1->CR1 &= ~TIM_CR1_CEN; // Disable timer
            TIM1->DIER &= ~(TIM_DIER_UDE | TIM_DIER_CC1DE | TIM_DIER_CC2DE); // Disable DMA requests
            
            // Disable PWM outputs so the pin actually stays LOW
            TIM1->CCER &= ~(TIM_CCER_CC1E | TIM_CCER_CC2E);
            
            // Force all GPIO low immediately
            GPIOB->BSRR = (uint32_t)ws2812_gpio_set_bits << 16;

            // Clear interrupt flags before enabling UIE to prevent premature trigger
            TIM1->CR1 &= ~(TIM_CR1_UDIS | TIM_CR1_URS); // Enable update events (clear UDIS) and allow any source to trigger updates (clear URS)
            TIM1->ARR = RESET_TIMER_PERIOD - 1;         // Auto-Reload Register to define timer period (subtract 1 because counter is 0-based)
            // Generate Update Event to load ARR, then CLEAR the flag it generates
            TIM1->EGR = TIM_EGR_UG;     // Manually trigger Update Event to immediately load ARR into the shadow register
            TIM1->SR = 0;               // Clear all status flags, including UIF set by the Update Event above
            TIM1->DIER |= TIM_DIER_UIE; // Enable Update Interrupt (will fire when counter reaches ARR and reloads)
            TIM1->CR1 |= TIM_CR1_CEN;   // Start the timer by setting Counter Enable bit
            
        } else {
            for (int i = 0; i < DMA_BUFFER_FILL_SIZE; i += 8) {
                fill_dma_buffer(dma_buffer + DMA_BUFFER_FILL_SIZE + i, framebuffer_position, led_channels);
                framebuffer_position++;
            }
        }
    }
    
    // Check for errors
    if (DMA2->LISR & DMA_LISR_TEIF2) { // Transfer error
        DMA2->LIFCR = DMA_LIFCR_CTEIF2; // Clear transfer error flag
        hal_dma_error_flag++;
    }
    // DBG_PB9_TOGGLE();
}


void TIM1_UP_TIM10_Handler(void)
{
    if (TIM1->SR & TIM_SR_UIF) { // Update interrupt flag
        TIM1->SR &= ~TIM_SR_UIF; // Clear update interrupt flag
        
        // Reset period finished! 
        TIM1->CR1 &= ~TIM_CR1_CEN; // Stop timer
        TIM1->DIER &= ~TIM_DIER_UIE; // Disable interrupt
        
        // Re-enable PWM outputs for the next refresh call
        TIM1->CCER |= (TIM_CCER_CC1E | TIM_CCER_CC2E); // Enable channel 1 and 2 outputs
    }
}

void ws2812_refresh(const struct led_channel_info *channels, GPIO_TypeDef *gpio_bank)
{
    DBG_PB9_TOGGLE();
    TIM1->EGR |= TIM_EGR_UG; // Generate update event to load registers
    uint32_t i;

    for (i = 0; i < WS2812_NUM_CHANNELS; i++) {
        led_channels[i] = channels[i];
    }

    TIM1->CR1 &= ~TIM_CR1_CEN; // Disable timer
    
    // Disable DMA streams
    DMA2_Stream1->CR &= ~DMA_SxCR_EN; // Disable DMA stream 1
    DMA2_Stream2->CR &= ~DMA_SxCR_EN; // Disable DMA stream 2
    DMA2_Stream5->CR &= ~DMA_SxCR_EN; // Disable DMA stream 5
    
    TIM1->DIER &= ~(TIM_DIER_UDE | TIM_DIER_CC1DE | TIM_DIER_CC2DE); // Disable DMA requests

    channels_max_length = 0;
    framebuffer_position = 0;
    ws2812_gpio_set_bits = 0;

    // Pre-fill buffer
    for (i = 0; i < DMA_BUFFER_SIZE; i += 8) {
        fill_dma_buffer(dma_buffer + i, framebuffer_position, led_channels);
        framebuffer_position++;
    }

    for (i = 0; i < WS2812_NUM_CHANNELS; i++) {
        if (channels[i].length_in_bytes > channels_max_length) {
            channels_max_length = channels[i].length_in_bytes;
        }
        if (channels[i].length_in_bytes != 0) {
            ws2812_gpio_set_bits |= (1 << ws2812_channel_gpio_map[i]);
        }
    }

    channels_max_length += DMA_BUFFER_SIZE / 8;

#if !WS212_ALL_CHANNELS_SAME_LENGTH
    channels_max_length += 3;
#endif

    // Clear DMA flags
    DMA2->LIFCR = DMA_LIFCR_CTEIF1 | DMA_LIFCR_CHTIF1 | DMA_LIFCR_CTCIF1; // Stream 1
    DMA2->LIFCR = DMA_LIFCR_CTEIF2 | DMA_LIFCR_CHTIF2 | DMA_LIFCR_CTCIF2; // Stream 2
    DMA2->HIFCR = DMA_HIFCR_CTEIF5 | DMA_HIFCR_CHTIF5 | DMA_HIFCR_CTCIF5; // Stream 5

    // Clear timer flags and disable update interrupt
    TIM1->SR = 0; // Clear all status flags
    TIM1->DIER &= ~TIM_DIER_UIE; // Disable update interrupt

    // Set DMA transfer counts
    DMA2_Stream1->NDTR = DMA_BUFFER_SIZE; // Stream 1
    DMA2_Stream2->NDTR = DMA_BUFFER_SIZE; // Stream 2
    DMA2_Stream5->NDTR = DMA_BUFFER_SIZE; // Stream 5

    // Enable DMA streams
    DMA2_Stream1->CR |= DMA_SxCR_EN; // Stream 1
    DMA2_Stream2->CR |= DMA_SxCR_EN; // Stream 2
    DMA2_Stream5->CR |= DMA_SxCR_EN; // Stream 5

    TIM1->DIER |= TIM_DIER_UDE | TIM_DIER_CC1DE | TIM_DIER_CC2DE; // Enable DMA requests

    // Set period and start timer
    TIM1->ARR = WS2812_TIMER_PERIOD; // Auto-Reload Register to define timer period
    TIM1->CNT = TIM1->ARR; // Start timer (potentially do ARR-10 just before the overflow to trigger DMA immediately???)
    TIM1->CR1 |= TIM_CR1_CEN; // Start the timer by setting Counter Enable bit

    DBG_PB9_TOGGLE();
}

void ws2812_init(void)
{
    ws2812_gpio_init();
    ws2812_timer1_init();
    ws2812_dma_init();
}

