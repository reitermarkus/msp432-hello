TOOLCHAIN_BIN ?= /Applications/TI/ccs/tools/compiler/gcc-arm-none-eabi-7-2017-q4-major/bin
LD = $(TOOLCHAIN_BIN)/arm-none-eabi-ld
OBJCOPY = $(TOOLCHAIN_BIN)/arm-none-eabi-objcopy

TARGET = thumbv7em-none-eabihf
TARGET_UPPERCASE = $(shell echo $(TARGET) | tr [:lower:] [:upper:] | tr - _)

export CARGO_TARGET_$(TARGET_UPPERCASE)_LINKER = $(LD)

.PHONY: debug
debug:
	cargo build --target $(TARGET) -vvv
	"$(OBJCOPY)" -O binary target/$(TARGET)/debug/msp432-newio target/$(TARGET)/debug/msp432-newio.bin

.PHONY: release
release:
	cargo build --target $(TARGET) --release
	"$(OBJCOPY)" -O binary target/$(TARGET)/release/msp432-newio target/$(TARGET)/release/msp432-newio.bin

.PHONY: flash
flash: debug
	dslite --config MSP432P401R.ccxml --verbose --flash --verify target/$(TARGET)/debug/msp432-newio.bin,0x0

.PHONY: run
run: flash

.PHONY: clean
clean:
	cargo clean
