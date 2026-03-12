# 平台和 SBI
BOARD := qemu
SBI ?= rustsbi

# bootloader
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# Rust 目标架构
TARGET := riscv64gc-unknown-none-elf

# 编译输出
MODE := debug
KERNEL := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL).bin

# 编译
build:
	cargo build --target $(TARGET)

# 生成裸 binary
binary: build
	riscv64-unknown-elf-objcopy \
	-O binary \
	$(KERNEL) \
	$(KERNEL_BIN)

# 运行
run: binary
	qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-bios $(BOOTLOADER) \
	-kernel $(KERNEL_BIN)

# 清理
clean:
	cargo clean