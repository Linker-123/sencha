global main
extern printf,exit
section .data
format db "%#llu", 10, 0

section .bss
sum dq ?

section .text
main:
mov rax,9007199254740991
mov rbx,9007199254740991
add rax,rbx
mov [sum],rax
mov rsi,[sum]
lea rdi,[rel format]
xor rax,rax
call printf
xor rax,rax
mov rax,1
int 0x80
