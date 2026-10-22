.section .rodata
coord_x_prompt: .asciz "X: "
coord_y_prompt: .asciz "Y: "
driver1: .asciz "Олександр Шевченко (DRV-1001) | Skoda Octavia | AA1234BC"
driver2: .asciz "Володимир Зеленський (DRV-1002) | Potuzhno Leet | AA7777AA"
driver3: .asciz "Кирило Буданов (DRV-1003) | Skoda Kodiaq | KA1337KA"
driver4: .asciz "Михайло Запобіжний (DRV-1004) | BMW i8 | AA9931BC"
driver5: .asciz "Мухамед Браун (DRV-1005) | LADA Priora Sedan | AA1344BC"
driver6: .asciz "Євгеній Руйда (DRV-1006) | Volkswagen Tiguan | AA1214BC"
driver_name1: .asciz "Олександр Шевченко"
driver_name2: .asciz "Володимир Зеленський"
driver_name3: .asciz "Кирило Буданов"
driver_name4: .asciz "Михайло Запобіжний"
driver_name5: .asciz "Мухамед Браун"
driver_name6: .asciz "Євгеній Руйда"
driver_id1: .asciz "DRV-1001"
driver_id2: .asciz "DRV-1002"
driver_id3: .asciz "DRV-1003"
driver_id4: .asciz "DRV-1004"
driver_id5: .asciz "DRV-1005"
driver_id6: .asciz "DRV-1006"
.align 8
driver_names: .quad driver_name1, driver_name2, driver_name3, driver_name4, driver_name5, driver_name6
driver_ids: .quad driver_id1, driver_id2, driver_id3, driver_id4, driver_id5, driver_id6
/* 64-byte record: description, class, available, rating, trips, x, y, id */
drivers:
    .quad driver1, 1, 1
    .double 4.92
    .quad 318
    .double 2.5, 3.1
    .quad 0
    .quad driver2, 4, 0
    .double 4.37
    .quad 57
    .double 1.2, 0.8
    .quad 1
    .quad driver3, 3, 1
    .double 4.67
    .quad 146
    .double 4.2, 4.9
    .quad 2
    .quad driver4, 3, 1
    .double 4.72
    .quad 89
    .double 8.0, 1.3
    .quad 3
    .quad driver5, 1, 1
    .double 4.52
    .quad 23
    .double 4.5, 3.1
    .quad 4
    .quad driver6, 2, 1
    .double 4.99
    .quad 512
    .double 1.5, 7.1
    .quad 5
.section .bss
.align 8
pickup_x: .zero 8
pickup_y: .zero 8
selected_driver: .zero 8
pickup_distance: .zero 8

FUNC read_order
    ASK "Номер замовлення: "
    cmp BYTE PTR [rax], 0
    je .Lorder_input_bad
    lea rdi, [rip + order_id]
    call save_string
    lea rdi, [rip + coord_x_prompt]
    call read_coordinate
    movsd [rip + pickup_x], xmm0
    lea rdi, [rip + coord_y_prompt]
    call read_coordinate
    movsd [rip + pickup_y], xmm0
.Lclass_again:
    INTEGER "Клас авто (1 Економ, 2 Стандарт, 3 Комфорт, 4 Бізнес): "
    test edx, edx
    jz .Lclass_bad
    cmp rax, 1
    jl .Lclass_bad
    cmp rax, 4
    jg .Lclass_bad
    mov [rip + car_class], rax
    mov eax, 1
    END
.Lclass_bad:
    call input_error
    jmp .Lclass_again
.Lorder_input_bad:
    call input_error
    xor eax, eax
    END

/* hypot avoids overflow from squaring large but finite coordinates. */
FUNC match_driver
    sub rsp, 16
    lea rax, [rip + drivers]
    mov [rbp-8], rax
    mov QWORD PTR [rbp-16], 6
    mov QWORD PTR [rip + selected_driver], 0
    movsd xmm0, [rip + ten]
    movsd [rip + pickup_distance], xmm0
.Lcandidate:
    mov rcx, [rbp-8]
    cmp QWORD PTR [rcx+16], 0
    je .Lcandidate_next
    mov rax, [rip + car_class]
    cmp [rcx+8], rax
    jne .Lcandidate_next
    movsd xmm0, [rcx+40]
    subsd xmm0, QWORD PTR [rip + pickup_x]
    movsd xmm1, [rcx+48]
    subsd xmm1, QWORD PTR [rip + pickup_y]
    call hypot
    ucomisd xmm0, QWORD PTR [rip + pickup_distance]
    ja .Lcandidate_next
    jp .Lcandidate_next
    jb .Lcandidate_save
    cmp QWORD PTR [rip + selected_driver], 0
    jne .Lcandidate_next
.Lcandidate_save:
    movsd [rip + pickup_distance], xmm0
    mov rcx, [rbp-8]
    mov [rip + selected_driver], rcx
.Lcandidate_next:
    add QWORD PTR [rbp-8], 64
    dec QWORD PTR [rbp-16]
    jnz .Lcandidate
    mov rcx, [rip + selected_driver]
    test rcx, rcx
    jz .Lno_driver
    mov rsi, [rcx]
    movsd xmm0, [rcx+24]
    PRINT "Водій: %s | Рейтинг: %.2f\n", 1
    movsd xmm0, [rip + pickup_distance]
    movapd xmm1, xmm0
    mulsd xmm1, QWORD PTR [rip + eta_factor]
    PRINT "Відстань до пасажира: %.2f км | Час подачі: ~%.0f хв\n", 2
    mov eax, 1
    END
.Lno_driver:
    SAY "Немає вільних водіїв цього класу в радіусі 10 км. Замовлення скасовано."
    xor eax, eax
    END
FUNC matching_run
    call read_order
    test eax, eax
    jz .Lmatching_done
    call match_driver
.Lmatching_done:
    END
