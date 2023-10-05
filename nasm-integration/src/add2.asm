section .note.gnu-stack
global add2

section .text

add2:
    push rbp
    mov rbp, rsp

    mov eax, edi
    add eax, 2

    leave
    ret
