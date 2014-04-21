        .globl _put32
_put32: str r1, [r0]
        bx lr

        .globl _get32
_get32: ldr r0, [r0]
        bx lr
