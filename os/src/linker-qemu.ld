OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* 内核加载地址 */
    . = BASE_ADDRESS;
    skernel = .;

    /* 代码段 */
    .text : ALIGN(4K) {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        etext = .;
    }

    /* 只读数据 */
    .rodata : ALIGN(4K) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        erodata = .;
    }

    /* 可写数据 */
    .data : ALIGN(4K) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        edata = .;
    }

    /* 未初始化数据 */
    .bss : ALIGN(4K) {
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(.bss.stack)
        ebss = .;
    }

    /* 内核结束 */
    ekernel = .;

    /* 丢弃调试信息 */
    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
        *(.comment)
        *(.symtab)
        *(.strtab)
        *(.riscv.attributes)
    }
}