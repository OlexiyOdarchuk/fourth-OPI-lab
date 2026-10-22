.section .rodata
.align 8
platinum_threshold: .double 4.90
gold_threshold: .double 4.75
standard_threshold: .double 4.50
compliment_delimiters: .asciz " ,\t\r\n"
compliment1: .asciz "Чисте та доглянуте авто"
compliment2: .asciz "Ввічливість і клієнтоорієнтованість"
compliment3: .asciz "Безпечне і комфортне керування"
compliment4: .asciz "Пунктуальність та оптимальний маршрут"
compliment5: .asciz "Базовий рівень обслуговування"
compliment_names: .quad compliment1, compliment2, compliment3, compliment4, compliment5
.section .bss
.align 8
rating_id: .zero 8
rating_name: .zero 8
old_rating: .zero 8
new_rating: .zero 8
previous_trips: .zero 8
stars: .zero 8
compliments: .zero 8
comment: .zero 8
token_state: .zero 8

FUNC rating_run
    ASK "Ідентифікатор водія: "
    cmp BYTE PTR [rax], 0
    je .Lrating_bad
    lea rdi, [rip + rating_id]
    call save_string
    ASK "Повне ім'я водія: "
    cmp BYTE PTR [rax], 0
    jne .Lrating_name
    lea rax, [rip + default_driver_name]
.Lrating_name:
    lea rdi, [rip + rating_name]
    call save_string
    NUMBER "Поточний рейтинг (1.00–5.00): "
    test edx, edx
    jz .Lrating_bad
    ucomisd xmm0, QWORD PTR [rip + one]
    jb .Lrating_bad
    ucomisd xmm0, QWORD PTR [rip + five]
    ja .Lrating_bad
    movsd [rip + old_rating], xmm0
    INTEGER "Кількість попередніх оцінених поїздок: "
    test edx, edx
    jz .Lrating_bad
    mov ecx, 4294967294
    cmp rax, rcx
    ja .Lrating_bad
    mov [rip + previous_trips], rax
    call rating_feedback
    END
.Lrating_bad:
    call input_error
    END

FUNC rating_feedback
    sub rsp, 16
    INTEGER "Оцінка поїздки (1–5): "
    test edx, edx
    jz .Lfeedback_bad
    cmp rax, 1
    jl .Lfeedback_bad
    cmp rax, 5
    jg .Lfeedback_bad
    mov [rip + stars], rax
    ASK "Компліменти через пробіл/кому (1 чистота, 2 ввічливість, 3 безпека, 4 швидкість, 5 стандарт): "
    mov QWORD PTR [rip + compliments], 0
    mov rdi, rax
.Ltoken_next:
    lea rsi, [rip + compliment_delimiters]
    lea rdx, [rip + token_state]
    call strtok_r
    test rax, rax
    jz .Ltokens_done
    call parse_integer
    test edx, edx
    jz .Ltoken_bad
    cmp rax, 1
    jl .Ltoken_bad
    cmp rax, 5
    jg .Ltoken_bad
    dec rax
    bts QWORD PTR [rip + compliments], rax
    jmp .Ltoken_continue
.Ltoken_bad:
    SAY "[УВАГА] Невідому категорію пропущено."
.Ltoken_continue:
    xor edi, edi
    jmp .Ltoken_next
.Ltokens_done:
    cmp QWORD PTR [rip + compliments], 0
    jne .Lread_comment
    mov QWORD PTR [rip + compliments], 16
.Lread_comment:
    ASK "Текстовий відгук (можна порожній): "
    lea rdi, [rip + comment]
    call save_string
    mov rax, [rip + previous_trips]
    cvtsi2sd xmm0, rax
    mulsd xmm0, QWORD PTR [rip + old_rating]
    cvtsi2sd xmm1, QWORD PTR [rip + stars]
    addsd xmm0, xmm1
    inc rax
    cvtsi2sd xmm1, rax
    divsd xmm0, xmm1
    mulsd xmm0, QWORD PTR [rip + thousand]
    addsd xmm0, QWORD PTR [rip + half]
    cvttsd2si rax, xmm0
    cvtsi2sd xmm0, rax
    divsd xmm0, QWORD PTR [rip + thousand]
    movsd [rip + new_rating], xmm0
    mov rsi, [rip + rating_id]
    mov rdx, [rip + rating_name]
    PRINT "\nКАРТКА ОЦІНЮВАННЯ | %s | %s\n"
    movsd xmm0, [rip + new_rating]
    movapd xmm1, xmm0
    subsd xmm1, QWORD PTR [rip + old_rating]
    PRINT "Оновлений рейтинг: %.3f (зміна: %+.3f)\n", 2
    mov rsi, [rip + previous_trips]
    inc rsi
    PRINT "Всього оцінок: %ld\n"
    mov rax, [rip + previous_trips]
    inc rax
    cmp rax, 10
    jl .Ltier_new
    movsd xmm0, [rip + new_rating]
    ucomisd xmm0, QWORD PTR [rip + platinum_threshold]
    jae .Ltier_platinum
    ucomisd xmm0, QWORD PTR [rip + gold_threshold]
    jae .Ltier_gold
    ucomisd xmm0, QWORD PTR [rip + standard_threshold]
    jae .Ltier_standard
    SAY "Risk Tier | Потребує переатестації | Пріоритет: -20.0 %\n[УВАГА] Рейтинг нижче 4.50: попередження диспетчеру (симуляція)."
    jmp .Ltier_done
.Ltier_new:
    SAY "Новий водій (Newcomer) | Рівень визначається після 10 оцінок | Пріоритет: +0.0 %"
    jmp .Ltier_done
.Ltier_platinum:
    SAY "Platinum Tier | Вищий пілотаж сервісу | Пріоритет: +15.0 %"
    jmp .Ltier_done
.Ltier_gold:
    SAY "Gold Tier | Надійний перевізник | Пріоритет: +8.0 %"
    jmp .Ltier_done
.Ltier_standard:
    SAY "Standard Tier | Відповідає базовим вимогам | Пріоритет: +0.0 %"
.Ltier_done:
    mov QWORD PTR [rbp-8], 0
.Lprint_compliment:
    mov rcx, [rbp-8]
    bt QWORD PTR [rip + compliments], rcx
    jnc .Lprint_compliment_next
    lea rax, [rip + compliment_names]
    mov rdi, [rax+rcx*8]
    call puts
.Lprint_compliment_next:
    inc QWORD PTR [rbp-8]
    cmp QWORD PTR [rbp-8], 5
    jl .Lprint_compliment
    mov rsi, [rip + comment]
    PRINT "Відгук: %s\n"
    END
.Lfeedback_bad:
    call input_error
    END
.section .rodata
default_driver_name: .asciz "Володимир Ковалюк"
