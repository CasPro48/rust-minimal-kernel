/* Linker script for x86_64 kernel */
ENTRY(_start)

SECTIONS
{
    . = 1M;

    .boot :
    {
        /* Multiboot header */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .data :
    {
        *(.data .data.*)
    }

    .bss :
    {
        *(COMMON)
        *(.bss .bss.*)
    }

    /DISCARD/ :
    {
        *(.eh_frame)
        *(.note.*)
    }
}
