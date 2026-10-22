#include "common.inc"
#include "input.S"
#include "fare.S"
#include "matching.S"
#include "rating.S"
#include "report.S"
#include "order.S"

.global main
FUNC main
.Lmenu:
    SAY "\nСИСТЕМА TAXI MANAGEMENT\n1. Оцінювання поїздки\n2. Генерація звітів\n3. Обрахунок вартості поїздки\n4. Підбір водія\n5. Оформлення замовлення (повний цикл)\n0. Вихід"
    INTEGER "Оберіть дію: "
    test edx, edx
    jz .Lbad_menu
    test rax, rax
    jz finish
    cmp rax, 1
    je .Lrating
    cmp rax, 2
    je .Lreport
    cmp rax, 3
    je .Lfare
    cmp rax, 4
    je .Lmatching
    cmp rax, 5
    je .Lorder
.Lbad_menu:
    call input_error
    jmp .Lmenu
.Lrating:
    call rating_run
    jmp .Lmenu
.Lreport:
    call report_run
    jmp .Lmenu
.Lfare:
    call fare_run
    jmp .Lmenu
.Lmatching:
    call matching_run
    jmp .Lmenu
.Lorder:
    call order_run
    jmp .Lmenu

/* EOF at any prompt terminates without looping or recording partial work. */
.text
finish:
    /* Reached by jumps from nested frames, never returns. */
    and rsp, -16
    SAY "\nРоботу завершено."
    xor edi, edi
    call exit
.section .note.GNU-stack,"",@progbits
