CC      := clang
AR      := llvm-ar
RM      := rm -rf
MKDIR   := mkdir -p

# Project Folders
BUILD_DIR  := ../target/ext/ws28xx
CUBE_PATH  := STM32CubeF4
CMSIS_PATH := $(CUBE_PATH)/Drivers/CMSIS
HAL_PATH   := $(CUBE_PATH)/Drivers/STM32F4xx_HAL_Driver

# Target Library
TARGET := $(BUILD_DIR)/libws28xx.a

# Compilation Flags
CFLAGS += --target=thumbv7em-none-eabi \
		  -mcpu=cortex-m4 \
		  -mthumb \
		  -mfpu=fpv4-sp-d16 \
		  -mfloat-abi=hard \
		  -fcommon \
		  -ffunction-sections \
		  -fdata-sections \
		  -DSTM32F429xx \
		  -flto \

# Include Directories
INCLUDES := -I. \
			-Isrc \
			-I$(CMSIS_PATH)/Include \
			-I$(CMSIS_PATH)/Device/ST/STM32F4xx/Include \
			-I$(HAL_PATH)/Inc

# Source Files
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

# Map sources to object files in the build directory
OBJECTS := $(addprefix $(BUILD_DIR)/, $(notdir $(SOURCES:.c=.o)))
OBJECTS += $(addprefix $(BUILD_DIR)/, $(HAL_SRCS:.c=.o))

# Build Rules
.PHONY: all clean check_dirs

all: check_dirs $(TARGET)

# Create build directory if it doesn't exist
check_dirs:
	@$(MKDIR) $(BUILD_DIR)

# Link: Create static library
$(TARGET): $(OBJECTS)
	@echo "Creating static library: $@"
	$(AR) rcs $@ $(OBJECTS)

# Compile: Local src/ files
$(BUILD_DIR)/%.o: src/%.c
	@echo "Compiling: $<"
	@echo "CFLAGS: $(CFLAGS)"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

# Compile: HAL Driver files
$(BUILD_DIR)/%.o: $(HAL_PATH)/Src/%.c
	@echo "Compiling HAL: $<"
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

clean:
	@echo "Cleaning build directory..."
	$(RM) $(BUILD_DIR)