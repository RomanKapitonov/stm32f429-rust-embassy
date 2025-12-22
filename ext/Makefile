CC      := clang
AR      := llvm-ar
RM      := rm -rf
MKDIR   := mkdir -p

BUILD_DIR  := ../target/ext/ws28xx
CUBE_PATH  := STM32CubeF4
CMSIS_PATH := $(CUBE_PATH)/Drivers/CMSIS
HAL_PATH   := $(CUBE_PATH)/Drivers/STM32F4xx_HAL_Driver

TARGET := $(BUILD_DIR)/libws28xx.a

# Some compiler flags are propagated from build.rs
# See build.rs for details
CFLAGS += --target=thumbv7em-none-eabihf \
		  -mcpu=cortex-m4 \
		  -mthumb \
		  -mfpu=fpv4-sp-d16 \
		  -mfloat-abi=hard \
		  -fcommon \
		  -ffunction-sections \
		  -fdata-sections \
		  -DSTM32F429xx \
		  -flto \

INCLUDES := -I. \
			-Isrc \
			-I$(CMSIS_PATH)/Include \
			-I$(CMSIS_PATH)/Device/ST/STM32F4xx/Include \
			-I$(HAL_PATH)/Inc

SOURCES := src/ws28xx.c
HAL_SRCS := stm32f4xx_hal.c \
			stm32f4xx_hal_cortex.c \
			stm32f4xx_hal_dma.c \
			stm32f4xx_hal_gpio.c \
			stm32f4xx_hal_rcc.c \
			stm32f4xx_hal_rcc_ex.c \
			stm32f4xx_hal_pwr.c \
			stm32f4xx_hal_pwr_ex.c \
			stm32f4xx_hal_tim.c \
			stm32f4xx_hal_tim_ex.c

OBJECTS := $(addprefix $(BUILD_DIR)/, $(notdir $(SOURCES:.c=.o)))
OBJECTS += $(addprefix $(BUILD_DIR)/, $(HAL_SRCS:.c=.o))

.PHONY: all clean check_dirs

all: check_dirs $(TARGET)

check_dirs:
	@$(MKDIR) $(BUILD_DIR)

$(TARGET): $(OBJECTS)
	@echo "Creating static library: $@"
	$(AR) rcs $@ $(OBJECTS)

$(BUILD_DIR)/%.o: src/%.c
	@echo "Compiling: $<"
	@echo "CFLAGS: $(CFLAGS)"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

$(BUILD_DIR)/%.o: $(HAL_PATH)/Src/%.c
	@echo "Compiling HAL: $<"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

clean:
	@echo "Cleaning build directory..."
	$(RM) $(BUILD_DIR)