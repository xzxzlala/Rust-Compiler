section .data
        HEAP:    times 1024 dq 0
section .text
        global start_here
        extern snake_error
        extern print_snake_val
start_here:
        push R15
        sub RSP, 8
        lea r15, [rel HEAP]
        call main
        add rsp, 8
        pop r15
        ret
lambda0:
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -32], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -40], rax
;;; let_def_end
;;; let_body
;;; make_array
        mov QWORD [r15 + 0], 2
        mov rax, QWORD [rsp + -32]
        mov QWORD [r15 + 8], rax
        mov rax, QWORD [rsp + -40]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 1
        add r15, 24
;;; let_body_end
;;; let_body_end
        ret
lambda1:
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -24], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 0
        mov QWORD [rsp + -32], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -32]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -24]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        ret
lambda2:
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -24], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 2
        mov QWORD [rsp + -32], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -32]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -24]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        ret
lambda3:
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -24], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 0
        mov QWORD [rsp + -32], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -32]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -24]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -40], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -48], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -40]
        mov QWORD [rsp + -56], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -48]
;;; eq or neq
        mov r10, rax
        mov rax, QWORD [rsp + -56]
        cmp r10, rax
        mov rax, 0xffffffffffffffff
        jz eq_done28
        mov rax, 0x7fffffffffffffff
eq_done28:
;;; let_body_end
;;; let_body_end
;;; let_body_end
        ret
range0:
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -40], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -32]
        mov QWORD [rsp + -48], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -40]
;;; gt_lt_ge_le
        test rax, 0x00000001
        jnz error_com_not_number
        sar rax, 0x00000001
        mov r10, rax
        mov rax, QWORD [rsp + -48]
        test rax, 0x00000001
        jnz error_com_not_number
        sar rax, 0x00000001
        sub r10, rax
        mov rax, r10
        mov r11, 0x8000000000000000
        and rax, r11
        mov r11, 0x7fffffffffffffff
        or rax, r11
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -56], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -56]
        mov r9, rax
        xor r9, 0x00000007
        test r9, 0x00000007
        jnz error_if_not_boolean
;;; cond
        mov r10, 0x7fffffffffffffff
        cmp rax, r10
        je if_false36
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -64], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -64]
        mov r9, rax
        xor r9, 0x00000003
        test r9, 0x00000007
        jnz error_not_closure
        sub rax, 3
        mov r10, QWORD [rax + 0]
        cmp r10, 0x00000002
        jne error_wrong_arity
        add rax, 3
;;; let_body_end
        mov QWORD [rsp + -72], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -72]
        mov QWORD [rsp + -80], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -80]
        sub rax, 3
        mov rax, QWORD [rax + 8]
;;; let_body_end
        mov QWORD [rsp + -88], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -72]
        mov QWORD [rsp + -96], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -96]
        sub rax, 3
        mov rax, QWORD [rax + 16]
;;; let_body_end
        mov QWORD [rsp + -104], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -104]
        mov QWORD [rsp + -112], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -120], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -128], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -136], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -144], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 2
        mov QWORD [rsp + -152], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -144]
;;; add_sub_mul
        test rax, 0x00000001
        jnz error_ari_not_number
        mov r10, rax
        sar r10, 0x00000001
        mov rax, QWORD [rsp + -152]
        test rax, 0x00000001
        jnz error_ari_not_number
        sar rax, 0x00000001
        mov r11, rax
        mov rax, r10
        mov r10, r11
        add rax, r10
        jo error_overflow
        shl rax, 0x00000001
        jo error_overflow
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -160], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -32]
        mov QWORD [rsp + -168], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -128]
        mov QWORD [rsp + -184], rax
        mov rax, QWORD [rsp + -136]
        mov QWORD [rsp + -192], rax
        mov rax, QWORD [rsp + -160]
        mov QWORD [rsp + -200], rax
        mov rax, QWORD [rsp + -168]
        mov QWORD [rsp + -208], rax
        sub rsp, 168
        call range0
        add rsp, 168
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -176], rax
;;; let_def_end
;;; let_body
        mov r8, QWORD [rsp + -88]
        mov rax, QWORD [rsp + -112]
        mov QWORD [rsp + -8], rax
        mov rax, QWORD [rsp + -120]
        mov QWORD [rsp + -16], rax
        mov rax, QWORD [rsp + -176]
        mov QWORD [rsp + -24], rax
        jmp r8
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        jmp done36
if_false36:
        mov rax, QWORD [rsp + -8]
done36:
;;; let_body_end
        ret
range0_closure:
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -32], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 2
        mov QWORD [rsp + -40], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -40]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -32]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -48], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -56], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 0
        mov QWORD [rsp + -64], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -64]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -56]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -72], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -80], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -88], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -48]
        mov QWORD [rsp + -104], rax
        mov rax, QWORD [rsp + -72]
        mov QWORD [rsp + -112], rax
        mov rax, QWORD [rsp + -80]
        mov QWORD [rsp + -120], rax
        mov rax, QWORD [rsp + -88]
        mov QWORD [rsp + -128], rax
        sub rsp, 88
        call range0
        add rsp, 88
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        ret
foldl0:
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -56], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -56]
        mov r9, rax
        xor r9, 0x00000003
        test r9, 0x00000007
        jnz error_not_closure
        sub rax, 3
        mov r10, QWORD [rax + 0]
        cmp r10, 0x00000001
        jne error_wrong_arity
        add rax, 3
;;; let_body_end
        mov QWORD [rsp + -64], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -64]
        mov QWORD [rsp + -72], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -72]
        sub rax, 3
        mov rax, QWORD [rax + 8]
;;; let_body_end
        mov QWORD [rsp + -80], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -64]
        mov QWORD [rsp + -88], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -88]
        sub rax, 3
        mov rax, QWORD [rax + 16]
;;; let_body_end
        mov QWORD [rsp + -96], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -96]
        mov QWORD [rsp + -104], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -48]
        mov QWORD [rsp + -112], rax
;;; let_def_end
;;; let_body
        mov r8, QWORD [rsp + -80]
        mov rax, QWORD [rsp + -104]
        mov QWORD [rsp + -128], rax
        mov rax, QWORD [rsp + -112]
        mov QWORD [rsp + -136], rax
        sub rsp, 112
        call r8
        add rsp, 112
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -120], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -120]
        mov r9, rax
        xor r9, 0x00000007
        test r9, 0x00000007
        jnz error_if_not_boolean
;;; cond
        mov r10, 0x7fffffffffffffff
        cmp rax, r10
        je if_false106
        mov rax, QWORD [rsp + -40]
        jmp done106
if_false106:
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -128], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -136], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -144], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -32]
        mov QWORD [rsp + -152], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -32]
        mov QWORD [rsp + -160], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -160]
        mov r9, rax
        xor r9, 0x00000003
        test r9, 0x00000007
        jnz error_not_closure
        sub rax, 3
        mov r10, QWORD [rax + 0]
        cmp r10, 0x00000002
        jne error_wrong_arity
        add rax, 3
;;; let_body_end
        mov QWORD [rsp + -168], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -168]
        mov QWORD [rsp + -176], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -176]
        sub rax, 3
        mov rax, QWORD [rax + 8]
;;; let_body_end
        mov QWORD [rsp + -184], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -168]
        mov QWORD [rsp + -192], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -192]
        sub rax, 3
        mov rax, QWORD [rax + 16]
;;; let_body_end
        mov QWORD [rsp + -200], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -200]
        mov QWORD [rsp + -208], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -40]
        mov QWORD [rsp + -216], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -224], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -224]
        mov r9, rax
        xor r9, 0x00000003
        test r9, 0x00000007
        jnz error_not_closure
        sub rax, 3
        mov r10, QWORD [rax + 0]
        cmp r10, 0x00000001
        jne error_wrong_arity
        add rax, 3
;;; let_body_end
        mov QWORD [rsp + -232], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -232]
        mov QWORD [rsp + -240], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -240]
        sub rax, 3
        mov rax, QWORD [rax + 8]
;;; let_body_end
        mov QWORD [rsp + -248], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -232]
        mov QWORD [rsp + -256], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -256]
        sub rax, 3
        mov rax, QWORD [rax + 16]
;;; let_body_end
        mov QWORD [rsp + -264], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -264]
        mov QWORD [rsp + -272], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -48]
        mov QWORD [rsp + -280], rax
;;; let_def_end
;;; let_body
        mov r8, QWORD [rsp + -248]
        mov rax, QWORD [rsp + -272]
        mov QWORD [rsp + -296], rax
        mov rax, QWORD [rsp + -280]
        mov QWORD [rsp + -304], rax
        sub rsp, 280
        call r8
        add rsp, 280
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -288], rax
;;; let_def_end
;;; let_body
        mov r8, QWORD [rsp + -248]
        mov rax, QWORD [rsp + -208]
        mov QWORD [rsp + -304], rax
        mov rax, QWORD [rsp + -216]
        mov QWORD [rsp + -312], rax
        mov rax, QWORD [rsp + -288]
        mov QWORD [rsp + -320], rax
        sub rsp, 288
        call r8
        add rsp, 288
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -296], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -304], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -304]
        mov r9, rax
        xor r9, 0x00000003
        test r9, 0x00000007
        jnz error_not_closure
        sub rax, 3
        mov r10, QWORD [rax + 0]
        cmp r10, 0x00000001
        jne error_wrong_arity
        add rax, 3
;;; let_body_end
        mov QWORD [rsp + -312], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -312]
        mov QWORD [rsp + -320], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -320]
        sub rax, 3
        mov rax, QWORD [rax + 8]
;;; let_body_end
        mov QWORD [rsp + -328], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -312]
        mov QWORD [rsp + -336], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -336]
        sub rax, 3
        mov rax, QWORD [rax + 16]
;;; let_body_end
        mov QWORD [rsp + -344], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -344]
        mov QWORD [rsp + -352], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -48]
        mov QWORD [rsp + -360], rax
;;; let_def_end
;;; let_body
        mov r8, QWORD [rsp + -328]
        mov rax, QWORD [rsp + -352]
        mov QWORD [rsp + -376], rax
        mov rax, QWORD [rsp + -360]
        mov QWORD [rsp + -384], rax
        sub rsp, 360
        call r8
        add rsp, 360
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -368], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -128]
        mov QWORD [rsp + -8], rax
        mov rax, QWORD [rsp + -136]
        mov QWORD [rsp + -16], rax
        mov rax, QWORD [rsp + -144]
        mov QWORD [rsp + -24], rax
        mov rax, QWORD [rsp + -152]
        mov QWORD [rsp + -32], rax
        mov rax, QWORD [rsp + -296]
        mov QWORD [rsp + -40], rax
        mov rax, QWORD [rsp + -368]
        mov QWORD [rsp + -48], rax
        jmp foldl0
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
done106:
;;; let_body_end
        ret
foldl0_closure:
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -40], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 4
        mov QWORD [rsp + -48], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -48]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -40]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -56], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -64], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 2
        mov QWORD [rsp + -72], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -72]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -64]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -80], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -88], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 0
        mov QWORD [rsp + -96], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -96]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -88]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -104], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -112], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -120], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -32]
        mov QWORD [rsp + -128], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -56]
        mov QWORD [rsp + -144], rax
        mov rax, QWORD [rsp + -80]
        mov QWORD [rsp + -152], rax
        mov rax, QWORD [rsp + -104]
        mov QWORD [rsp + -160], rax
        mov rax, QWORD [rsp + -112]
        mov QWORD [rsp + -168], rax
        mov rax, QWORD [rsp + -120]
        mov QWORD [rsp + -176], rax
        mov rax, QWORD [rsp + -128]
        mov QWORD [rsp + -184], rax
        sub rsp, 128
        call foldl0
        add rsp, 128
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        ret
lambda5:
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -32], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -40], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -32]
;;; add_sub_mul
        test rax, 0x00000001
        jnz error_ari_not_number
        mov r10, rax
        sar r10, 0x00000001
        mov rax, QWORD [rsp + -40]
        test rax, 0x00000001
        jnz error_ari_not_number
        sar rax, 0x00000001
        mov r11, rax
        mov rax, r10
        mov r10, r11
        imul rax, r10
        jo error_overflow
        shl rax, 0x00000001
        jo error_overflow
;;; let_body_end
;;; let_body_end
        ret
lambda4:
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -24], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 8
        mov QWORD [rsp + -32], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -32]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -24]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -40], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -48], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 6
        mov QWORD [rsp + -56], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -56]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -48]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -64], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -72], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 4
        mov QWORD [rsp + -80], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -80]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -72]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -88], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -96], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 2
        mov QWORD [rsp + -104], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -104]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -96]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -112], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -120], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 0
        mov QWORD [rsp + -128], rax
;;; let_def_end
;;; let_body
;;; array_get
        mov rax, QWORD [rsp + -128]
        mov r14, rax
        test r14, 0x00000001
        jnz error_index_not_number
        mov rax, QWORD [rsp + -120]
        mov r9, rax
        xor r9, 0x00000001
        test r9, 0x00000007
        jnz error_index_into_nonarray
        sub rax, 1
        sar r14, 0x00000001
        cmp r14, 0
        jl error_index_out_of_bound
        cmp r14, QWORD [rax + 0]
        jge error_index_out_of_bound
        mov rax, QWORD [rax + r14 * 8 + 8]
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -136], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -88]
        mov QWORD [rsp + -144], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -112]
        mov QWORD [rsp + -152], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -136]
        mov QWORD [rsp + -160], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; make_array
        mov QWORD [r15 + 0], 0
        mov rax, r15
        add rax, 1
        add r15, 8
        mov QWORD [rsp + -168], rax
;;; let_def_end
;;; let_body
;;; make_closure
        mov QWORD [r15 + 0], 0x00000002
        lea r9, [rel lambda5]
        mov QWORD [r15 + 8], r9
        mov rax, QWORD [rsp + -168]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 3
        add r15, 24
;;; let_body_end
        mov QWORD [rsp + -176], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 2
        mov QWORD [rsp + -184], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -40]
        mov QWORD [rsp + -192], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -64]
        mov QWORD [rsp + -200], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 2
        mov QWORD [rsp + -208], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -16]
        mov QWORD [rsp + -216], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 2
        mov QWORD [rsp + -224], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -216]
;;; add_sub_mul
        test rax, 0x00000001
        jnz error_ari_not_number
        mov r10, rax
        sar r10, 0x00000001
        mov rax, QWORD [rsp + -224]
        test rax, 0x00000001
        jnz error_ari_not_number
        sar rax, 0x00000001
        mov r11, rax
        mov rax, r10
        mov r10, r11
        add rax, r10
        jo error_overflow
        shl rax, 0x00000001
        jo error_overflow
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -232], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -192]
        mov QWORD [rsp + -248], rax
        mov rax, QWORD [rsp + -200]
        mov QWORD [rsp + -256], rax
        mov rax, QWORD [rsp + -208]
        mov QWORD [rsp + -264], rax
        mov rax, QWORD [rsp + -232]
        mov QWORD [rsp + -272], rax
        sub rsp, 232
        call range0
        add rsp, 232
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -240], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -144]
        mov QWORD [rsp + -256], rax
        mov rax, QWORD [rsp + -152]
        mov QWORD [rsp + -264], rax
        mov rax, QWORD [rsp + -160]
        mov QWORD [rsp + -272], rax
        mov rax, QWORD [rsp + -176]
        mov QWORD [rsp + -280], rax
        mov rax, QWORD [rsp + -184]
        mov QWORD [rsp + -288], rax
        mov rax, QWORD [rsp + -240]
        mov QWORD [rsp + -296], rax
        sub rsp, 240
        call foldl0
        add rsp, 240
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        ret
main:
;;; let_def
        mov rax, 0x7fffffffffffffff
        mov QWORD [rsp + -8], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; make_array
        mov QWORD [r15 + 0], 0
        mov rax, r15
        add rax, 1
        add r15, 8
        mov QWORD [rsp + -16], rax
;;; let_def_end
;;; let_body
;;; make_closure
        mov QWORD [r15 + 0], 0x00000002
        lea r9, [rel lambda0]
        mov QWORD [r15 + 8], r9
        mov rax, QWORD [rsp + -16]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 3
        add r15, 24
;;; let_body_end
        mov QWORD [rsp + -24], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; make_array
        mov QWORD [r15 + 0], 0
        mov rax, r15
        add rax, 1
        add r15, 8
        mov QWORD [rsp + -32], rax
;;; let_def_end
;;; let_body
;;; make_closure
        mov QWORD [r15 + 0], 0x00000001
        lea r9, [rel lambda1]
        mov QWORD [r15 + 8], r9
        mov rax, QWORD [rsp + -32]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 3
        add r15, 24
;;; let_body_end
        mov QWORD [rsp + -40], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; make_array
        mov QWORD [r15 + 0], 0
        mov rax, r15
        add rax, 1
        add r15, 8
        mov QWORD [rsp + -48], rax
;;; let_def_end
;;; let_body
;;; make_closure
        mov QWORD [r15 + 0], 0x00000001
        lea r9, [rel lambda2]
        mov QWORD [r15 + 8], r9
        mov rax, QWORD [rsp + -48]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 3
        add r15, 24
;;; let_body_end
        mov QWORD [rsp + -56], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -64], rax
;;; let_def_end
;;; let_body
;;; make_array
        mov QWORD [r15 + 0], 1
        mov rax, QWORD [rsp + -64]
        mov QWORD [r15 + 8], rax
        mov rax, r15
        add rax, 1
        add r15, 16
;;; let_body_end
        mov QWORD [rsp + -72], rax
;;; let_def_end
;;; let_body
;;; make_closure
        mov QWORD [r15 + 0], 0x00000001
        lea r9, [rel lambda3]
        mov QWORD [r15 + 8], r9
        mov rax, QWORD [rsp + -72]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 3
        add r15, 24
;;; let_body_end
        mov QWORD [rsp + -80], rax
;;; let_def_end
;;; let_body
;;; fun_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -88], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -96], rax
;;; let_def_end
;;; let_body
;;; make_array
        mov QWORD [r15 + 0], 2
        mov rax, QWORD [rsp + -88]
        mov QWORD [r15 + 8], rax
        mov rax, QWORD [rsp + -96]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 1
        add r15, 24
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -104], rax
;;; let_def_end
;;; let_body
;;; make_closure
        mov QWORD [r15 + 0], 0x00000002
        lea r9, [rel range0_closure]
        mov QWORD [r15 + 8], r9
        mov rax, QWORD [rsp + -104]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 3
        add r15, 24
;;; let_body_end
        mov QWORD [rsp + -112], rax
;;; let_def_end
;;; let_body
;;; fun_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -56]
        mov QWORD [rsp + -120], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -40]
        mov QWORD [rsp + -128], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -80]
        mov QWORD [rsp + -136], rax
;;; let_def_end
;;; let_body
;;; make_array
        mov QWORD [r15 + 0], 3
        mov rax, QWORD [rsp + -120]
        mov QWORD [r15 + 8], rax
        mov rax, QWORD [rsp + -128]
        mov QWORD [r15 + 16], rax
        mov rax, QWORD [rsp + -136]
        mov QWORD [r15 + 24], rax
        mov rax, r15
        add rax, 1
        add r15, 32
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -144], rax
;;; let_def_end
;;; let_body
;;; make_closure
        mov QWORD [r15 + 0], 0x00000003
        lea r9, [rel foldl0_closure]
        mov QWORD [r15 + 8], r9
        mov rax, QWORD [rsp + -144]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 3
        add r15, 24
;;; let_body_end
        mov QWORD [rsp + -152], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -80]
        mov QWORD [rsp + -160], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -40]
        mov QWORD [rsp + -168], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -56]
        mov QWORD [rsp + -176], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -24]
        mov QWORD [rsp + -184], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -8]
        mov QWORD [rsp + -192], rax
;;; let_def_end
;;; let_body
;;; make_array
        mov QWORD [r15 + 0], 5
        mov rax, QWORD [rsp + -160]
        mov QWORD [r15 + 8], rax
        mov rax, QWORD [rsp + -168]
        mov QWORD [r15 + 16], rax
        mov rax, QWORD [rsp + -176]
        mov QWORD [r15 + 24], rax
        mov rax, QWORD [rsp + -184]
        mov QWORD [r15 + 32], rax
        mov rax, QWORD [rsp + -192]
        mov QWORD [r15 + 40], rax
        mov rax, r15
        add rax, 1
        add r15, 48
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -200], rax
;;; let_def_end
;;; let_body
;;; make_closure
        mov QWORD [r15 + 0], 0x00000001
        lea r9, [rel lambda4]
        mov QWORD [r15 + 8], r9
        mov rax, QWORD [rsp + -200]
        mov QWORD [r15 + 16], rax
        mov rax, r15
        add rax, 3
        add r15, 24
;;; let_body_end
        mov QWORD [rsp + -208], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -208]
        mov QWORD [rsp + -216], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -216]
        mov r9, rax
        xor r9, 0x00000003
        test r9, 0x00000007
        jnz error_not_closure
        sub rax, 3
        mov r10, QWORD [rax + 0]
        cmp r10, 0x00000001
        jne error_wrong_arity
        add rax, 3
;;; let_body_end
        mov QWORD [rsp + -224], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -224]
        mov QWORD [rsp + -232], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -232]
        sub rax, 3
        mov rax, QWORD [rax + 8]
;;; let_body_end
        mov QWORD [rsp + -240], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -224]
        mov QWORD [rsp + -248], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -248]
        sub rax, 3
        mov rax, QWORD [rax + 16]
;;; let_body_end
        mov QWORD [rsp + -256], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -256]
        mov QWORD [rsp + -264], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 6
        mov QWORD [rsp + -272], rax
;;; let_def_end
;;; let_body
        mov r8, QWORD [rsp + -240]
        mov rax, QWORD [rsp + -264]
        mov QWORD [rsp + -288], rax
        mov rax, QWORD [rsp + -272]
        mov QWORD [rsp + -296], rax
        sub rsp, 272
        call r8
        add rsp, 272
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -280], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -208]
        mov QWORD [rsp + -288], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -288]
        mov r9, rax
        xor r9, 0x00000003
        test r9, 0x00000007
        jnz error_not_closure
        sub rax, 3
        mov r10, QWORD [rax + 0]
        cmp r10, 0x00000001
        jne error_wrong_arity
        add rax, 3
;;; let_body_end
        mov QWORD [rsp + -296], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -296]
        mov QWORD [rsp + -304], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -304]
        sub rax, 3
        mov rax, QWORD [rax + 8]
;;; let_body_end
        mov QWORD [rsp + -312], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -296]
        mov QWORD [rsp + -320], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -320]
        sub rax, 3
        mov rax, QWORD [rax + 16]
;;; let_body_end
        mov QWORD [rsp + -328], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -328]
        mov QWORD [rsp + -336], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 8
        mov QWORD [rsp + -344], rax
;;; let_def_end
;;; let_body
        mov r8, QWORD [rsp + -312]
        mov rax, QWORD [rsp + -336]
        mov QWORD [rsp + -360], rax
        mov rax, QWORD [rsp + -344]
        mov QWORD [rsp + -368], rax
        sub rsp, 344
        call r8
        add rsp, 344
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -352], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -208]
        mov QWORD [rsp + -360], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -360]
        mov r9, rax
        xor r9, 0x00000003
        test r9, 0x00000007
        jnz error_not_closure
        sub rax, 3
        mov r10, QWORD [rax + 0]
        cmp r10, 0x00000001
        jne error_wrong_arity
        add rax, 3
;;; let_body_end
        mov QWORD [rsp + -368], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -368]
        mov QWORD [rsp + -376], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -376]
        sub rax, 3
        mov rax, QWORD [rax + 8]
;;; let_body_end
        mov QWORD [rsp + -384], rax
;;; let_def_end
;;; let_body
;;; let_def
;;; let_def
        mov rax, QWORD [rsp + -368]
        mov QWORD [rsp + -392], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -392]
        sub rax, 3
        mov rax, QWORD [rax + 16]
;;; let_body_end
        mov QWORD [rsp + -400], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, QWORD [rsp + -400]
        mov QWORD [rsp + -408], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 10
        mov QWORD [rsp + -416], rax
;;; let_def_end
;;; let_body
        mov r8, QWORD [rsp + -384]
        mov rax, QWORD [rsp + -408]
        mov QWORD [rsp + -432], rax
        mov rax, QWORD [rsp + -416]
        mov QWORD [rsp + -440], rax
        sub rsp, 416
        call r8
        add rsp, 416
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        mov QWORD [rsp + -424], rax
;;; let_def_end
;;; let_body
;;; make_array
        mov QWORD [r15 + 0], 3
        mov rax, QWORD [rsp + -280]
        mov QWORD [r15 + 8], rax
        mov rax, QWORD [rsp + -352]
        mov QWORD [r15 + 16], rax
        mov rax, QWORD [rsp + -424]
        mov QWORD [r15 + 24], rax
        mov rax, r15
        add rax, 1
        add r15, 32
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; fun_body_end
;;; let_body_end
;;; fun_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
;;; let_body_end
        ret

error_ari_not_number:
        mov rsi, rax
        mov rdi, 1
        call snake_error
error_com_not_number:
        mov RSI, RAX
        mov RDI, 2
        call snake_error
error_overflow:
        mov rsi, rax
        mov rdi, 3
        call snake_error
error_if_not_boolean:
        mov RSI, RAX
        mov RDI, 4
        call snake_error
error_logic_not_boolean:
        mov RSI, RAX
        mov RDI, 5
        call snake_error
error_index_not_number:
        mov RSI, RAX
        mov RDI, 6
        call snake_error
error_index_out_of_bound:
        mov RSI, RAX
        mov RDI, 7
        call snake_error
error_index_into_nonarray:
        mov RSI, RAX
        mov RDI, 8
        call snake_error
error_length_into_nonarray:
        mov RSI, RAX
        mov RDI, 9
        call snake_error
error_not_closure:
        mov RSI, RAX
        mov RDI, 10
        call snake_error
error_wrong_arity:
        mov RSI, R10
        mov RDI, 11
        call snake_error
