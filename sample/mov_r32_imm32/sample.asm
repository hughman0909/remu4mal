BITS 32
    org 0x7c00         ; loads on address 0x7c00 
    mov eax, 268435456 ; mov r32, imm32
    jmp 0              ; RIP jumps to 0

; expected: RAX = 0x10000000
