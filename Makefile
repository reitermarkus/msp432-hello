TARGET = thumbv7em-none-eabihf

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
	openocd -f 'openocd.cfg' -c 'init' -c 'reset halt' -c 'flash write_image erase target/$(TARGET)/release/msp432-hello 0x0' -c 'reset run' -c 'shutdown'

.PHONY: dslite
dslite: release
	"$(TOOLCHAIN_BIN)/arm-none-eabi-objcopy" -O binary target/$(TARGET)/release/msp432-hello target/$(TARGET)/release/msp432-hello.bin
	dslite --config MSP432P401R.ccxml --verbose --flash --verify target/$(TARGET)/release/msp432-hello.bin,0x0

.PHONY: run
run: debug
	 cargo run --target $(TARGET)

.PHONY: clean
clean:
	cargo clean
