FUNC order_run
    sub rsp, 16
    call read_order
    test eax, eax
    jz .Lflow_done
    SAY "Пункт призначення:"
    lea rdi, [rip + coord_x_prompt]
    call read_coordinate
    movsd [rbp-8], xmm0
    lea rdi, [rip + coord_y_prompt]
    call read_coordinate
    movapd xmm1, xmm0
    subsd xmm1, QWORD PTR [rip + pickup_y]
    movsd xmm0, [rbp-8]
    subsd xmm0, QWORD PTR [rip + pickup_x]
    call hypot
    ucomisd xmm0, QWORD PTR [rip + zero]
    jbe .Lflow_zero
    movsd [rip + distance], xmm0
    mulsd xmm0, QWORD PTR [rip + eta_factor]
    movsd [rip + duration], xmm0
    call match_driver
    test eax, eax
    jz .Lflow_done
    ASK "Телефон пасажира: "
    lea rdi, [rip + phone]
    call save_string
    call read_payment
    call settle
    test eax, eax
    jz .Lflow_done
    /* Commit before feedback, as in the original: bad/absent feedback
     * does not undo a paid ride. The six-driver bitset counts unique IDs. */
    mov rax, [rip + total]
    add [rip + session_revenue], rax
    inc QWORD PTR [rip + session_count]
    mov rcx, [rip + selected_driver]
    mov rax, [rcx+56]
    bts QWORD PTR [rip + session_drivers], rax
    movsd xmm0, [rcx+24]
    movsd [rip + old_rating], xmm0
    mov rax, [rcx+32]
    mov [rip + previous_trips], rax
    mov rdx, [rcx+56]
    lea rax, [rip + driver_names]
    mov rax, [rax+rdx*8]
    lea rdi, [rip + rating_name]
    call save_string
    mov rcx, [rip + selected_driver]
    mov rdx, [rcx+56]
    lea rax, [rip + driver_ids]
    mov rax, [rax+rdx*8]
    lea rdi, [rip + rating_id]
    call save_string
    call rating_feedback
.Lflow_done:
    END
.Lflow_zero:
    SAY "[ПОМИЛКА] Пункт призначення збігається з місцем посадки."
    END
