.section .rodata
date_start_prompt: .asciz "Дата початку (ДД-ММ-РРРР): "
date_end_prompt: .asciz "Дата завершення (ДД-ММ-РРРР): "
yes_session_prompt: .asciz "Використати замовлення поточного сеансу? (y/n): "
yes_data_prompt: .asciz "Є дані для обраного звіту? (y/n): "
yes_report_prompt: .asciz "Сформувати інший звіт? (y/n): "
count_prompt: .asciz "Кількість виконаних замовлень: "
revenue_prompt: .asciz "Сумарна виручка (грн): "
drivers_prompt: .asciz "Кількість водіїв: "
hours_prompt: .asciz "Сумарний час на лінії (год): "
busy_prompt: .asciz "Час виконання замовлень (год): "
month_days: .byte 31,28,31,30,31,30,31,31,30,31,30,31
.section .bss
.align 8
report_type: .zero 8
start_date: .zero 8
end_date: .zero 8
report_session: .zero 8
report_count: .zero 8
report_revenue: .zero 8
report_drivers: .zero 8
report_hours: .zero 8
report_busy: .zero 8

/* Validate Gregorian dates, including leap centuries. Return YYYYMMDD.
 * Strict 10-byte DD-MM-YYYY input avoids libc timezone/date normalization. */
FUNC read_date
    sub rsp, 16
    mov [rbp-8], rdi
.Ldate_again:
    mov rdi, [rbp-8]
    call read_line
    mov [rbp-16], rax
    mov rdi, rax
    call strlen
    cmp rax, 10
    jne .Ldate_bad
    mov rsi, [rbp-16]
    cmp BYTE PTR [rsi+2], '-'
    jne .Ldate_bad
    cmp BYTE PTR [rsi+5], '-'
    jne .Ldate_bad
    xor ecx, ecx
.Ldate_digits:
    cmp ecx, 2
    je .Ldate_digit_next
    cmp ecx, 5
    je .Ldate_digit_next
    movzx eax, BYTE PTR [rsi+rcx]
    sub eax, '0'
    cmp eax, 9
    ja .Ldate_bad
.Ldate_digit_next:
    inc ecx
    cmp ecx, 10
    jl .Ldate_digits
    movzx r8d, BYTE PTR [rsi]
    sub r8d, '0'
    imul r8d, 10
    movzx eax, BYTE PTR [rsi+1]
    sub eax, '0'
    add r8d, eax
    movzx r9d, BYTE PTR [rsi+3]
    sub r9d, '0'
    imul r9d, 10
    movzx eax, BYTE PTR [rsi+4]
    sub eax, '0'
    add r9d, eax
    xor r10d, r10d
    mov ecx, 6
.Lyear:
    imul r10d, 10
    movzx eax, BYTE PTR [rsi+rcx]
    sub eax, '0'
    add r10d, eax
    inc ecx
    cmp ecx, 10
    jl .Lyear
    cmp r9d, 1
    jl .Ldate_bad
    cmp r9d, 12
    jg .Ldate_bad
    cmp r8d, 1
    jl .Ldate_bad
    lea rcx, [rip + month_days]
    movzx r11d, BYTE PTR [rcx+r9-1]
    cmp r9d, 2
    jne .Ldate_day_check
    test r10d, 3
    jnz .Ldate_day_check
    mov eax, r10d
    xor edx, edx
    mov ecx, 100
    div ecx
    test edx, edx
    jnz .Lleap
    mov eax, r10d
    xor edx, edx
    mov ecx, 400
    div ecx
    test edx, edx
    jnz .Ldate_day_check
.Lleap:
    inc r11d
.Ldate_day_check:
    cmp r8d, r11d
    jg .Ldate_bad
    imul eax, r10d, 10000
    imul ecx, r9d, 100
    add eax, ecx
    add eax, r8d
    END
.Ldate_bad:
    SAY "[ПОМИЛКА] Введіть існуючу дату у форматі ДД-ММ-РРРР."
    jmp .Ldate_again

FUNC report_run
.Lreport_again:
    INTEGER "Тип звіту (1 усі, 2 замовлення, 3 виручка, 4 водії): "
    test edx, edx
    jz .Lreport_type_bad
    cmp rax, 1
    jl .Lreport_type_bad
    cmp rax, 4
    jg .Lreport_type_bad
    mov [rip + report_type], rax
    lea rdi, [rip + date_start_prompt]
    call read_date
    mov [rip + start_date], rax
.Lreport_end_date:
    lea rdi, [rip + date_end_prompt]
    call read_date
    cmp rax, [rip + start_date]
    jl .Lreport_end_bad
    mov [rip + end_date], rax
    mov QWORD PTR [rip + report_session], 0
    cmp QWORD PTR [rip + session_count], 0
    je .Lreport_manual
    lea rdi, [rip + yes_session_prompt]
    call read_yes_no
    mov [rip + report_session], rax
    test eax, eax
    jnz .Lreport_data
.Lreport_manual:
    lea rdi, [rip + yes_data_prompt]
    call read_yes_no
    test eax, eax
    jz .Lreport_empty
.Lreport_data:
    cmp QWORD PTR [rip + report_type], 2
    jg .Lreport_revenue
    mov rax, [rip + session_count]
    cmp QWORD PTR [rip + report_session], 0
    jne .Lreport_count_ready
    lea rdi, [rip + count_prompt]
    call read_count
.Lreport_count_ready:
    mov [rip + report_count], rax
.Lreport_revenue:
    cmp QWORD PTR [rip + report_type], 1
    je .Lread_revenue
    cmp QWORD PTR [rip + report_type], 3
    jne .Lreport_drivers
.Lread_revenue:
    cvtsi2sd xmm0, QWORD PTR [rip + session_revenue]
    divsd xmm0, QWORD PTR [rip + hundred]
    cmp QWORD PTR [rip + report_session], 0
    jne .Lreport_revenue_ready
    lea rdi, [rip + revenue_prompt]
    call read_nonnegative
.Lreport_revenue_ready:
    movsd [rip + report_revenue], xmm0
.Lreport_drivers:
    cmp QWORD PTR [rip + report_type], 1
    je .Lread_drivers
    cmp QWORD PTR [rip + report_type], 4
    jne .Lreport_print
.Lread_drivers:
    mov rdx, [rip + session_drivers]
    xor eax, eax
.Lcount_bits:
    test rdx, rdx
    jz .Ldriver_count_ready
    lea rcx, [rdx-1]
    and rdx, rcx
    inc eax
    jmp .Lcount_bits
.Ldriver_count_ready:
    cmp QWORD PTR [rip + report_session], 0
    jne .Lreport_drivers_ready
    lea rdi, [rip + drivers_prompt]
    call read_count
.Lreport_drivers_ready:
    mov [rip + report_drivers], rax
    mov QWORD PTR [rip + report_hours], 0
    mov QWORD PTR [rip + report_busy], 0
    test rax, rax
    jz .Lreport_print
    lea rdi, [rip + hours_prompt]
    call read_nonnegative
    movsd [rip + report_hours], xmm0
.Lreport_busy_again:
    lea rdi, [rip + busy_prompt]
    call read_nonnegative
    ucomisd xmm0, QWORD PTR [rip + report_hours]
    ja .Lreport_busy_bad
    movsd [rip + report_busy], xmm0
.Lreport_print:
    /* YYYYMMDD keys are printed explicitly so the report retains its period. */
    mov rsi, [rip + start_date]
    mov rdx, [rip + end_date]
    PRINT "\nЗвіт за період %08ld – %08ld (РРРРММДД)\n"
    cmp QWORD PTR [rip + report_type], 2
    jg .Lprint_revenue
    mov rsi, [rip + report_count]
    PRINT "Виконано замовлень: %ld\n"
.Lprint_revenue:
    cmp QWORD PTR [rip + report_type], 1
    je .Lprint_revenue_yes
    cmp QWORD PTR [rip + report_type], 3
    jne .Lprint_drivers
.Lprint_revenue_yes:
    movsd xmm0, [rip + report_revenue]
    PRINT "Виручка %.2f грн\n", 1
.Lprint_drivers:
    cmp QWORD PTR [rip + report_type], 1
    je .Lprint_drivers_yes
    cmp QWORD PTR [rip + report_type], 4
    jne .Lreport_continue
.Lprint_drivers_yes:
    mov rsi, [rip + report_drivers]
    PRINT "Водіїв за період: %ld\n"
    movsd xmm0, [rip + report_hours]
    ucomisd xmm0, QWORD PTR [rip + zero]
    jbe .Lworkload_undefined
    movsd xmm1, [rip + report_busy]
    divsd xmm1, xmm0
    mulsd xmm1, QWORD PTR [rip + hundred]
    movapd xmm0, xmm1
    PRINT "Завантаженість водіїв: %.1f %%\n", 1
    jmp .Lreport_continue
.Lworkload_undefined:
    SAY "Завантаженість не визначено: час на лінії дорівнює 0"
    jmp .Lreport_continue
.Lreport_empty:
    SAY "За обраний період дані відсутні."
.Lreport_continue:
    lea rdi, [rip + yes_report_prompt]
    call read_yes_no
    test eax, eax
    jnz .Lreport_again
    END
.Lreport_type_bad:
    call input_error
    jmp .Lreport_again
.Lreport_end_bad:
    SAY "[ПОМИЛКА] Дата завершення не може бути раніше дати початку."
    jmp .Lreport_end_date
.Lreport_busy_bad:
    SAY "[ПОМИЛКА] Час замовлень не може перевищувати час на лінії."
    jmp .Lreport_busy_again
