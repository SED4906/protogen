.extern interrupt_handler

.macro ISR_NOERRCODE isrN
    isr\isrN:
        cli
        pushq $\isrN
        jmp isr_no_err_stub
    .global isr\isrN
.endm

.macro ISR_ERRCODE isrN
    isr\isrN:
        cli
        pushq $\isrN
        jmp isr_common_stub
    .global isr\isrN
.endm

isr_no_err_stub:
    pushq $0xEA7BEEF
isr_common_stub:
    push %r15
    push %r14
    push %r13
    push %r12
    push %r11
    push %r10
    push %r9
    push %r8
    push %rbp
    pushq $0xDEFEA7
    push %rdi
    push %rsi
    push %rdx
    push %rcx
    push %rbx
    push %rax
    mov %rsp, %rdi
    add $0x90, %rdi
    mov -0x8(%rdi), %rsi
    mov -0x10(%rdi), %rdx
    call interrupt_handler
    pop %rax
    pop %rbx
    pop %rcx
    pop %rdx
    pop %rsi
    pop %rdi
    add $0x8, %rsp
    pop %rbp
    pop %r8
    pop %r9
    pop %r10
    pop %r11
    pop %r12
    pop %r13
    pop %r14
    pop %r15
    add $0x10, %rsp
    iretq

ISR_NOERRCODE 0
ISR_NOERRCODE 1
ISR_NOERRCODE 2
ISR_NOERRCODE 3
ISR_NOERRCODE 4
ISR_NOERRCODE 5
ISR_NOERRCODE 6
ISR_NOERRCODE 7 
ISR_ERRCODE   8 
ISR_NOERRCODE 9 
ISR_ERRCODE   10
ISR_ERRCODE   11
ISR_ERRCODE   12
ISR_ERRCODE   13
ISR_ERRCODE   14
ISR_NOERRCODE 15
ISR_NOERRCODE 16
ISR_ERRCODE   17
ISR_NOERRCODE 18
ISR_NOERRCODE 19
ISR_NOERRCODE 20
ISR_ERRCODE   21
ISR_NOERRCODE 22
ISR_NOERRCODE 23
ISR_NOERRCODE 24
ISR_NOERRCODE 25
ISR_NOERRCODE 26
ISR_NOERRCODE 27
ISR_NOERRCODE 28
ISR_ERRCODE   29
ISR_NOERRCODE 30
ISR_NOERRCODE 31

.set i, 32
.irp i,223
    ISR_NOERRCODE i
    .set i, i+1
.endr
