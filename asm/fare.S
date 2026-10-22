.section .rodata
.align 8
/* base, per km, per minute, minimum; all amounts in kopecks. */
tariffs:
    .quad 5500, 950, 180, 8000
    .quad 7000, 1150, 200, 9500
    .quad 8500, 1400, 250, 12000
    .quad 14000, 2200, 450, 20000
promo_knu: .asciz "KNU2026"
promo_short: .asciz "KNU"
promo_start: .asciz "START"
promo_new: .asciz "NEWUSER"
pay_card: .asciz "Банківська картка"
pay_cash: .asciz "Готівка"
pay_mobile: .asciz "Apple Pay / Google Pay"
payment_names: .quad pay_card, pay_cash, pay_mobile
.section .bss
.align 8
base_cost: .zero 8
km_cost: .zero 8
time_cost: .zero 8
subtotal: .zero 8
surged: .zero 8
discount: .zero 8
minimum: .zero 8
topup: .zero 8
total: .zero 8
vat: .zero 8
commission: .zero 8
net: .zero 8

FUNC fare_run
    ASK "Номер замовлення: "
    cmp BYTE PTR [rax], 0
    je .Lfare_bad
    lea rdi, [rip + order_id]
    call save_string
    ASK "Телефон пасажира: "
    lea rdi, [rip + phone]
    call save_string
    INTEGER "Тариф (1 Економ, 2 Стандарт, 3 Комфорт, 4 Бізнес): "
    test edx, edx
    jz .Ldefault_tariff
    cmp rax, 1
    jl .Ldefault_tariff
    cmp rax, 4
    jle .Ltariff_set
.Ldefault_tariff:
    mov eax, 2
.Ltariff_set:
    mov [rip + car_class], rax
    NUMBER "Дистанція (км): "
    test edx, edx
    jz .Lfare_bad
    ucomisd xmm0, QWORD PTR [rip + zero]
    jbe .Lfare_bad
    movsd [rip + distance], xmm0
    NUMBER "Час у дорозі (хв): "
    test edx, edx
    jz .Lfare_bad
    ucomisd xmm0, QWORD PTR [rip + zero]
    jbe .Lfare_bad
    movsd [rip + duration], xmm0
    call read_payment
    call settle
    END
.Lfare_bad:
    call input_error
    END

FUNC read_payment
    NUMBER "Коефіцієнт попиту (1.0–2.5): "
    test edx, edx
    jz .Lsurge_default
    ucomisd xmm0, QWORD PTR [rip + one]
    jb .Lsurge_default
    ucomisd xmm0, QWORD PTR [rip + max_surge]
    jbe .Lsurge_ok
.Lsurge_default:
    movsd xmm0, [rip + one]
.Lsurge_ok:
    movsd [rip + surge], xmm0
    ASK "Промокод (KNU2026, START, NEWUSER або -): "
    lea rdi, [rip + promo]
    call save_string
    mov rcx, rax
.Luppercase:
    movzx edx, BYTE PTR [rcx]
    test edx, edx
    jz .Lpayment
    cmp edx, 'a'
    jb .Lupper_next
    cmp edx, 'z'
    ja .Lupper_next
    sub BYTE PTR [rcx], 32
.Lupper_next:
    inc rcx
    jmp .Luppercase
.Lpayment:
    INTEGER "Оплата (1 картка, 2 готівка, 3 Apple/Google Pay): "
    test edx, edx
    jz .Lpayment_default
    cmp rax, 2
    je .Lpayment_ok
    cmp rax, 3
    je .Lpayment_ok
.Lpayment_default:
    mov eax, 1
.Lpayment_ok:
    mov [rip + payment], rax
    END

/* settle -> EAX=1 for a valid paid ride. Positive floating-point products
 * use floor(x+0.5), matching llround. Remaining money math is integer. */
FUNC settle
    movsd xmm0, [rip + distance]
    ucomisd xmm0, QWORD PTR [rip + zero]
    jbe .Lsettle_bad
    ucomisd xmm0, QWORD PTR [rip + thousand]
    ja .Lsettle_bad
    jp .Lsettle_bad
    movsd xmm1, [rip + duration]
    ucomisd xmm1, QWORD PTR [rip + zero]
    jbe .Lsettle_bad
    ucomisd xmm1, QWORD PTR [rip + max_minutes]
    ja .Lsettle_bad
    jp .Lsettle_bad
    mov rax, [rip + car_class]
    dec rax
    shl rax, 5
    lea rcx, [rip + tariffs]
    add rcx, rax
    mov rax, [rcx]
    mov [rip + base_cost], rax
    mov rax, [rcx+24]
    mov [rip + minimum], rax
    cvtsi2sd xmm2, QWORD PTR [rcx+8]
    mulsd xmm0, xmm2
    addsd xmm0, QWORD PTR [rip + half]
    cvttsd2si rax, xmm0
    mov [rip + km_cost], rax
    cvtsi2sd xmm2, QWORD PTR [rcx+16]
    mulsd xmm1, xmm2
    addsd xmm1, QWORD PTR [rip + half]
    cvttsd2si rdx, xmm1
    mov [rip + time_cost], rdx
    add rax, rdx
    add rax, [rip + base_cost]
    mov [rip + subtotal], rax
    cvtsi2sd xmm0, rax
    mulsd xmm0, QWORD PTR [rip + surge]
    addsd xmm0, QWORD PTR [rip + half]
    cvttsd2si rax, xmm0
    mov [rip + surged], rax
    mov QWORD PTR [rip + discount], 0
    mov rdi, [rip + promo]
    lea rsi, [rip + promo_knu]
    call strcmp
    test eax, eax
    jz .Lpercent_discount
    mov rdi, [rip + promo]
    lea rsi, [rip + promo_short]
    call strcmp
    test eax, eax
    jz .Lpercent_discount
    mov rdi, [rip + promo]
    lea rsi, [rip + promo_start]
    call strcmp
    test eax, eax
    jz .Lfixed_discount
    mov rdi, [rip + promo]
    lea rsi, [rip + promo_new]
    call strcmp
    test eax, eax
    jz .Lfixed_discount
    SAY "Промокод: знижка 0 грн (не використано або код невідомий)."
    jmp .Ldiscount_done
.Lpercent_discount:
    mov rax, [rip + surged]
    imul rax, 20
    add rax, 50
    xor edx, edx
    mov ecx, 100
    div rcx
    jmp .Ldiscount_store
.Lfixed_discount:
    mov rax, [rip + surged]
    shr rax, 1
    mov ecx, 4000
    cmp rax, rcx
    cmova rax, rcx
.Ldiscount_store:
    mov [rip + discount], rax
.Ldiscount_done:
    mov rax, [rip + surged]
    sub rax, [rip + discount]
    mov rcx, rax
    cmp rax, [rip + minimum]
    cmovl rax, [rip + minimum]
    mov [rip + total], rax
    sub rax, rcx
    mov [rip + topup], rax
    mov rax, [rip + total]
    imul rax, 20
    add rax, 60
    xor edx, edx
    mov ecx, 120
    div rcx
    mov [rip + vat], rax
    mov rax, [rip + total]
    imul rax, 20
    add rax, 50
    xor edx, edx
    mov ecx, 100
    div rcx
    mov [rip + commission], rax
    mov rcx, [rip + total]
    sub rcx, rax
    mov [rip + net], rcx
    SAY "\nЕЛЕКТРОННИЙ ЧЕК (навчальна симуляція оплати)"
    mov rsi, [rip + order_id]
    mov rdx, [rip + phone]
    PRINT "Замовлення: %s | Клієнт: %s\n"
    movsd xmm0, [rip + distance]
    movsd xmm1, [rip + duration]
    PRINT "Маршрут: %.2f км | Час: %.2f хв\n", 2
    MONEY "Подача", base_cost
    MONEY "Дистанція", km_cost
    MONEY "Час", time_cost
    MONEY "Базова сума", subtotal
    MONEY "З урахуванням попиту", surged
    MONEY "Знижка за промокодом", discount
    MONEY "Доплата до мінімального тарифу", topup
    MONEY "ДО ОПЛАТИ", total
    MONEY "ПДВ (20%%)", vat
    MONEY "Комісія сервісу (20%%)", commission
    MONEY "Нараховано перевізнику", net
    mov rax, [rip + payment]
    lea rcx, [rip + payment_names]
    mov rsi, [rcx+rax*8-8]
    PRINT "Спосіб оплати: %s | AUTH_OK\n"
    mov eax, 1
    END
.Lsettle_bad:
    SAY "[ПОМИЛКА] Дистанція має бути в межах (0; 1000] км, час — (0; 1440] хв."
    xor eax, eax
    END
