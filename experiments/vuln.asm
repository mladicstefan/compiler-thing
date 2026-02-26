section .text
    global vuln

vuln:
    push rbp
    mov rbp,  rsp
    sub rsp, 16

    lea rsi, [rdi]
    lea rdi, [rbp-16]

.copy_loop:
    mov al, [rsi]
    mov [rdi], al

    inc rsi
    inc rdi

    test al, al
    jnz .copy_loop

    leave
    ret



