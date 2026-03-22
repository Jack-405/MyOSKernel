#启用高级宏
.altmacro
#保存指定通用寄存器x[n]
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
#恢复指定通用寄存器x[n]
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
#保存上下文并进入内核
__alltraps:
    #sscratch保存用户栈指针
    csrrw sp, sscratch, sp
    # now sp->kernel stack, sscratch->user stack
    addi sp, sp, -34*8; 
    #保存通用寄存器
    sd x1, 1*8(sp)#ra
    sd x3, 3*8(sp)#全局指针gp
    .set n,5
    .rept 27#保存通用寄存器x5-x31,重复27次
        SAVE_GP %n
        .set n,n+1
    .endr
    #CSR 不能直接用 sd 存储。
    csrr t0, sstatus#保存sstatus,用于切换栈
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)#保存trapcontext里被破坏的sp
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp
    call trap_handler

__restore:
    # case1: start running app by __restore
    # case2: back to U after handling trap
    mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    addi sp, sp, 34*8
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp#换回来
    sret

