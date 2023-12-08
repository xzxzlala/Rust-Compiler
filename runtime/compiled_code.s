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
main:
;;; let_def
        mov rax, 0x000000007f800000
        shl rax, 0x00000020
        add rax, 5
        mov rax, rax
        mov QWORD [rsp + -8], rax
;;; let_def_end
;;; let_body
;;; let_def
        mov rax, 0x000000007f800000
        shl rax, 0x00000020
        add rax, 5
        mov rax, rax
        mov QWORD [rsp + -16], rax
;;; let_def_end
;;; let_body
        mov rax, QWORD [rsp + -8]
;;; add_sub_mul
        test rax, 0x00000001
        jnz error_ari_not_number
        mov r10, rax
        sar r10, 0x00000001
        mov rax, QWORD [rsp + -16]
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
error_fari_not_float:
        mov RSI, R10
        mov RDI, 12
        call snake_error
error_fcom_not_float:
        mov RSI, R10
        mov RDI, 13
        call snake_error
error_foverflow:
        mov RSI, R10
        mov RDI, 14
        call snake_error