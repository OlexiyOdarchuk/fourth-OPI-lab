/* read_line(prompt) -> trimmed, reusable UTF-8 buffer. getline grows it
 * without a fixed input limit. Call save_string for persistent fields. */
FUNC read_line
    mov rsi, rdi
    PRINT "%s"
    xor edi, edi
    call fflush
    lea rdi, [rip + line]
    lea rsi, [rip + line_capacity]
    mov rdx, QWORD PTR [rip + stdin]
    call getline
    test rax, rax
    js finish
    mov rdx, QWORD PTR [rip + line]
.Ltrim_end:
    test rax, rax
    jz .Ltrim_start
    movzx ecx, BYTE PTR [rdx + rax - 1]
    cmp ecx, 32
    ja .Ltrim_start
    mov BYTE PTR [rdx + rax - 1], 0
    dec rax
    jmp .Ltrim_end
.Ltrim_start:
    movzx ecx, BYTE PTR [rdx]
    test ecx, ecx
    jz .Ltrim_done
    cmp ecx, 32
    ja .Ltrim_done
    inc rdx
    jmp .Ltrim_start
.Ltrim_done:
    mov rax, rdx
    END

/* save_string(rax=source, rdi=&owned_pointer), replacing previous value. */
FUNC save_string
    sub rsp, 16
    mov [rbp-8], rdi
    mov rdi, rax
    call strdup
    test rax, rax
    jz fatal_memory
    mov [rbp-16], rax
    mov rcx, [rbp-8]
    mov rdi, [rcx]
    call free
    mov rcx, [rbp-8]
    mov rax, [rbp-16]
    mov [rcx], rax
    END
.text
fatal_memory:
    and rsp, -16
    SAY "[ПОМИЛКА] Недостатньо пам'яті."
    mov edi, 1
    call exit

/* Parsers return EDX=1 on success, 0 for invalid/overflow/non-finite input.
 * Reject trailing garbage and empty strings. Integer parser is decimal. */
FUNC parse_double
    sub rsp, 16
    mov [rbp-8], rax
    call __errno_location
    mov DWORD PTR [rax], 0
    mov [rbp-16], rax
    mov rdi, [rbp-8]
    lea rsi, [rip + parse_end]
    call strtod
    xor edx, edx
    mov rcx, [rbp-16]
    cmp DWORD PTR [rcx], 0
    jne .Ldouble_done
    mov rcx, [rip + parse_end]
    cmp rcx, [rbp-8]
    je .Ldouble_done
    cmp BYTE PTR [rcx], 0
    jne .Ldouble_done
    movq rax, xmm0
    and rax, QWORD PTR [rip + abs_mask]
    cmp rax, QWORD PTR [rip + max_finite]
    ja .Ldouble_done
    mov edx, 1
.Ldouble_done:
    END
FUNC parse_integer
    sub rsp, 16
    mov [rbp-8], rax
    call __errno_location
    mov DWORD PTR [rax], 0
    mov [rbp-16], rax
    mov rdi, [rbp-8]
    lea rsi, [rip + parse_end]
    mov edx, 10
    call strtol
    xor edx, edx
    mov rcx, [rbp-16]
    cmp DWORD PTR [rcx], 0
    jne .Lint_done
    mov rcx, [rip + parse_end]
    cmp rcx, [rbp-8]
    je .Lint_done
    cmp BYTE PTR [rcx], 0
    jne .Lint_done
    mov edx, 1
.Lint_done:
    END
FUNC input_error
    SAY "[ПОМИЛКА] Некоректні вхідні дані."
    END

FUNC read_coordinate
    sub rsp, 16
    mov [rbp-8], rdi
.Lcoordinate_again:
    mov rdi, [rbp-8]
    call read_line
    call parse_double
    test edx, edx
    jnz .Lcoordinate_ok
    call input_error
    jmp .Lcoordinate_again
.Lcoordinate_ok:
    END
FUNC read_nonnegative
    sub rsp, 16
    mov [rbp-8], rdi
.Lnonnegative_again:
    mov rdi, [rbp-8]
    call read_coordinate
    ucomisd xmm0, QWORD PTR [rip + zero]
    jae .Lnonnegative_ok
    call input_error
    jmp .Lnonnegative_again
.Lnonnegative_ok:
    END
FUNC read_count
    sub rsp, 16
    mov [rbp-8], rdi
.Lcount_again:
    mov rdi, [rbp-8]
    call read_line
    call parse_integer
    test edx, edx
    jz .Lcount_bad
    test rax, rax
    jns .Lcount_ok
.Lcount_bad:
    call input_error
    jmp .Lcount_again
.Lcount_ok:
    END
FUNC read_yes_no
    sub rsp, 16
    mov [rbp-8], rdi
.Lyes_again:
    mov rdi, [rbp-8]
    call read_line
    cmp BYTE PTR [rax+1], 0
    jne .Lyes_bad
    movzx eax, BYTE PTR [rax]
    or eax, 32
    cmp eax, 'y'
    je .Lyes
    cmp eax, 'n'
    je .Lno
.Lyes_bad:
    call input_error
    jmp .Lyes_again
.Lyes:
    mov eax, 1
    END
.Lno:
    xor eax, eax
    END
