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
        mov QWORD [r15 + 0], 0x40b00000
        mov QWORD [r15 + 8], 0x40b00000
        movss xmm0, dword [r15 + 0]
        movss xmm1, dword [r15 + 8]
        addps xmm0, xmm1  
        movss dword [r15 + 0], xmm0
        mov QWORD rax, [r15 + 0]
        shl rax, 0x20
        add rax, 5
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