/* Linux x86-64, System V ABI. Each function keeps RSP aligned to 16
 * before calls. Business state is session-local, in the .bss section.
 * libc provides terminal I/O and numeric parsing; all application logic
 * is implemented below in assembly. */
.intel_syntax noprefix
.macro FUNC name
.text
\name:
    push rbp
    mov rbp, rsp
.endm
.macro END
    leave
    ret
.endm
.macro SAY text
.pushsection .rodata
.Lmsg\@: .asciz "\text"
.popsection
    lea rdi, [rip + .Lmsg\@]
    call puts
.endm
.macro PRINT text, nargs=0
.pushsection .rodata
.Lfmt\@: .asciz "\text"
.popsection
    lea rdi, [rip + .Lfmt\@]
    mov eax, \nargs
    call printf
.endm
.macro ASK text
.pushsection .rodata
.Lprompt\@: .asciz "\text"
.popsection
    lea rdi, [rip + .Lprompt\@]
    call read_line
.endm
.macro NUMBER text
    ASK "\text"
    call parse_double
.endm
.macro INTEGER text
    ASK "\text"
    call parse_integer
.endm
.macro MONEY label, value
    mov rax, QWORD PTR [rip + \value]
    xor edx, edx
    mov ecx, 100
    div rcx
    mov rsi, rax
    PRINT "\label: %ld.%02ld грн\n"
.endm
.section .rodata
.align 8
zero: .double 0.0
half: .double 0.5
one: .double 1.0
five: .double 5.0
ten: .double 10.0
hundred: .double 100.0
thousand: .double 1000.0
max_minutes: .double 1440.0
max_surge: .double 2.5
eta_factor: .double 1.7142857142857142
max_finite: .quad 0x7fefffffffffffff
abs_mask: .quad 0x7fffffffffffffff
.section .bss
.align 8
line: .zero 8
line_capacity: .zero 8
parse_end: .zero 8
order_id: .zero 8
phone: .zero 8
car_class: .zero 8
distance: .zero 8
duration: .zero 8
surge: .zero 8
promo: .zero 8
payment: .zero 8
session_count: .zero 8
session_revenue: .zero 8
session_drivers: .zero 8
