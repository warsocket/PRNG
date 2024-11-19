global _start

SYS_READ	equ 0
SYS_WRITE   equ 1
STDIN      	equ 0
STDOUT      equ 1
SYS_EXIT    equ 60
LARGEST_PRIME_64 equ 18446744073709551557
BSIZE		equ 512 ;Should be single bit so 1024 2048 etc (512 seems to work best, (also aligns with 4K pages 512*8 = 4k))


SECTION .bss
page resq BSIZE
seed resq 1


SECTION .text

_start:
; read seed in to r15

; mov rdi, STDIN
; lea rsi, seed
; mov rdx, 8
; xor rax, rax ; SYS_READ
; syscall

; mov r15, [rsi]

;STUB TO NOT ENTER EACH TIME DURING DEV
; mov r15, 0x0102030405060708
; mov r15, 0x89876363d18c506b
mov r15, 0x80876363d18c506d

; mov rcx, 100000000 ;100M
; mov rcx, 512
xor rcx, rcx

;Init
lea rbx, page
xor r14, r14

; ;store 8 intervals (fit in registers)
mov rax, 0xAAAAAAAAAAAAAAAA ;10101010
mov rdx, 0xCCCCCCCCCCCCCCCC ;11001100
mov rdi, 0xF0F0F0F0F0F0F0F0
mov rsi, 0xFF00FF00FF00FF00
mov r9, 0xFFFF0000FFFF0000
mov r10, 0xFFFFFFFF00000000
xor r11, r11
mov r12, 0xFFFFFFFFFFFFFFFF


; main loop
loop:

movq xmm0, rcx

;rcx (needed for rotation count)
;r13 is free
;r14 is free

;shift p0 signal for 0 bits value

mov r14, r15
xor r15, 0xFFFFFFFFFFFFFFFF	;bascially a 0 bit shift of all ones

;shift p1 signal for 1 bits value
mov rcx, 0x1
and rcx, r14
ror rax, cl
xor r15, rax
shr r14, 1

mov rcx, 0x3F
and rcx, r14
ror r15, cl
shr r14, 6


;shift p2 signal for 2 bits value
mov rcx, 0x3
and rcx, r14
ror rdx, cl
xor r15, rdx
shr r14, 2

mov rcx, 0x3F
and rcx, r14
ror r15, cl
shr r14, 6


;shift p4 signal for 3 bits value
mov rcx, 0x7
and rcx, r14
ror rdi, cl
xor r15, rdi
shr r14, 3

mov rcx, 0x3F
and rcx, r14
ror r15, cl
shr r14, 6


;shift p8 signal for 4 bits value
mov rcx, 0xF
and rcx, r14
ror rsi, cl
xor r15, rsi
shr r14, 4


mov rcx, 0x3F
and rcx, r14
ror r15, cl
shr r14, 6


;shift p16 signal for 5 bits value
mov rcx, 0x1F
and rcx, r14
ror r9, cl
xor r15, r9
shr r14, 5

mov rcx, 0x3F
and rcx, r14
ror r15, cl
shr r14, 6


;shift p32 signal for 6 bits value
mov rcx, 0x3F
and rcx, r14
ror r10, cl
xor r15, r10
shr r14, 6

mov rcx, 0x3F
and rcx, r14
ror r15, cl
shr r14, 6


;shift p64 signal for 7 bits value
mov rcx, 0x7F
and rcx, r14

;Special 128 bit ror (r12:r11) goes here
mov r13, r11
shrd r11, r12, cl
shrd r12, r13, cl
;;;;;;;;
xor r15, r11
; shr r14, 1 ;Not needed we dont need to clear r14
; reminder: we donly shift 6 bits in between xors of the bits

;Now we multiply by biggest 64-bit prime, and toss the overflow (keep the least significant 64 bits)
mov r13, rax
mov r14, rdx

mov rax, LARGEST_PRIME_64
mul r15
mov r15, rax

mov rdx, r14
mov rax, r13


movq rcx, xmm0


; jmp noback
; back:
; jmp loop
; noback:
; loop back

;write value
bswap r15
mov [rbx+rcx*8], r15
bswap r15

inc rcx
test rcx, BSIZE

jz loop


; ;write result
write:

;put xmm0 on stack
; push 0
; movq [rsp], xmm0

push rax
push rdx
push rdi
push rsi
push r9
push r10
push r11
push r12


mov rdi, STDOUT
lea rsi, [rbx]
mov rdx, 8*BSIZE
mov rax, SYS_WRITE
syscall


pop r12
pop r11
pop r10
pop r9
pop rsi
pop rdi
pop rdx
pop rax

;movq rcx, xmm0
; pop rcx; pop writtem xmm0 directly into rcx
xor rcx, rcx
jmp loop; for infinite loop

; jmp noback
; back:
; jmp loop
; noback:
; loop back


mov rax, SYS_EXIT
xor rdi, rdi
syscall