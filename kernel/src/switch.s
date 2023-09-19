.extern restore_register
.extern restore_stack
.extern restore_pagemap
.extern restore_rip
enter_task:
call restore_stack
pushq $0x23
push %rax
pushfq
push $0x1B
call restore_rip
push %rax
.set i, 0
.rept 7
    mov $i, %rdi
    call restore_register
    push %rax
    .set i, i+1
.endr
.set i, 8
.rept 8
    mov $i, %rdi
    call restore_register
    push %rax
    .set i, i+1
.endr
mov %rsp, %rbx
add $160, %rbx
call get_tss
mov %rbx, 4(%rax)
call restore_pagemap
mov %rax, %cr3
pop %r15
pop %r14
pop %r13
pop %r12
pop %r11
pop %r10
pop %r9
pop %r8
pop %rbp
pop %rdi
pop %rsi
pop %rdx
pop %rcx
pop %rbx
pop %rax
iretq
.global enter_task
