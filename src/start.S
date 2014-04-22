        .section ".init"
        .globl _start
_start: mov     sp,#0x8000
        bl      main
hang:   b       hang

        .globl _dummy
_dummy: bx      lr

        .globl _put32
_put32: str     r1, [r0]
        bx      lr

        .globl _get32
_get32: ldr     r0, [r0]
        bx      lr
