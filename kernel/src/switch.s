.extern restore_register
.extern restore_stack
.extern restore_pagemap
.extern restore_rip
.extern store_kernel_rip
enter_task:
mov %rsp, %rbx
add $160, %rbx
call get_tss
mov %rbx, 4(%rax)
pop %rdi
call store_kernel_rip
call restore_stack
pushq $0x23
push %rax
pushfq
pushq $0x1B
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

.extern store_register
.extern store_stack
.extern restore_kernel_pagemap
.extern restore_kernel_rip
.extern store_rip
exit_task:
push %rax
push %rbx
push %rcx
push %rdx
push %rsi
push %rdi
push %rbp
push %r8
push %r9
push %r10
push %r11
push %r12
push %r13
push %r14
push %r15
call timer_tick
cmp $0, %rax
jnz .nope
.set i, 0
.rept 7
    mov $i, %rdi
    pop %rsi
    call store_register
    .set i, i+1
.endr
.set i, 8
.rept 8
    mov $i, %rdi
    pop %rsi
    call store_register
    .set i, i+1
.endr
pop %rdi
call store_rip
pop %rdi
pop %rdi
pop %rdi
call store_stack
call restore_kernel_pagemap
pop %rdi
mov %rax, %cr3
call restore_kernel_rip
mov %rax, %rbx
mov %rsp, %rax
pushq $0x10
push %rax
pushfq
pushq $0x08
push %rbx
iretq
.nope:
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
.global exit_task
