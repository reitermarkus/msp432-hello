TOOLCHAIN_BIN ?= /Applications/TI/ccs/tools/compiler/gcc-arm-none-eabi-7-2017-q4-major/bin

TARGET = thumbv7em-none-eabihf
TARGET_UPPERCASE = $(shell echo $(TARGET) | tr [:lower:] [:upper:] | tr - _)

export CARGO_TARGET_$(TARGET_UPPERCASE)_LINKER = $(TOOLCHAIN_BIN)/arm-none-eabi-ld
export CARGO_TARGET_$(TARGET_UPPERCASE)_RUNNER = $(TOOLCHAIN_BIN)/arm-none-eabi-gdb -q -x debug.gdb

.PHONY: all
all: debug

.PHONY: doc
doc:
	cargo doc --target $(TARGET) --open

.PHONY: debug
debug:
	cargo build --target $(TARGET)

.PHONY: release
release:
	cargo build --target $(TARGET) --release

.PHONY: flash
flash: release
	openocd -f 'openocd.cfg' -c 'init' -c 'reset halt' -c 'flash write_image erase target/$(TARGET)/release/msp432-newio 0x0' -c 'reset run' -c 'shutdown'

.PHONY: dslite
dslite: release
	"$(TOOLCHAIN_BIN)/arm-none-eabi-objcopy" -O binary target/$(TARGET)/release/msp432-newio target/$(TARGET)/release/msp432-newio.bin
	dslite --config MSP432P401R.ccxml --verbose --flash --verify target/$(TARGET)/release/msp432-newio.bin,0x0

.PHONY: run
run: debug
	 openocd &
	 cargo run --target $(TARGET) || true
	 killall openocd

.PHONY: clean
clean:
	cargo clean
