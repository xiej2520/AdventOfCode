use std::arch::asm;
use std::fmt::Display;
pub fn run(input: &[u8]) -> impl Display {
    let mut res: i64;
    unsafe {
        asm!(
        r#"
2:
        push    rbp
        push    r15
        push    r14
        push    r13
        push    r12
        push    rbx
        mov     r8d, 50
        xor     ecx, ecx
        movabs  r9, 2951479051793528259
        xor     r10d, r10d
        jmp 5f
3:
        xor     edx, edx
        neg     rdx
4:
        add     r10, rax
        add     r10, 4
        add     r8, rdx
        add     r8, 100000000
        mov     rax, r8
        shr     rax, 2
        mul     r9
        shr     rdx, 2
        imul    rax, rdx, 100
        xor     edx, edx
        sub     r8, rax
        sete    dl
        add     rcx, rdx
        cmp     r10, rsi
        jae 33f
5:
        lea     r11, [rdi + r10]
        mov     rdx, -4
        mov     r15, -1
        mov     r12, -2
6:
        mov     r14, r15
        mov     rbx, r12
        lea     r15, [r10 + rdx]
        add     r15, 5
        lea     rax, [rdx + 1]
        cmp     r15, rsi
        ja 7f
        lea     r15, [r14 + 1]
        lea     r12, [rbx + 1]
        cmp     byte ptr [r11 + rdx + 4], 10
        mov     rdx, rax
        jne 6b
7:
        lea     rdx, [rax + r11]
        add     rdx, 3
        movzx   ebp, byte ptr [rdi + r10 + 1]
        cmp     byte ptr [rdi + r10], 76
        lea     r15, [rdi + r10 + 1]
        jne 8f
        cmp     bpl, 45
        jne 9f
        add     r11, 2
        cmp     r11, rdx
        jae 13f
        cmp     rax, 3
        jae 15f
        xor     edx, edx
        jmp 17f
8:
        cmp     bpl, 45
        jne 12f
        add     r11, 2
        cmp     r11, rdx
        jae 3b
        cmp     rax, 3
        jae 20f
        xor     edx, edx
        jmp 22f
9:
        cmp     r15, rdx
        jae 3b
        lea     rdx, [rax + 1]
        cmp     rdx, 3
        jae 24f
        xor     edx, edx
        jmp 26f
12:
        cmp     r15, rdx
        jae 14f
        lea     rdx, [rax + 1]
        cmp     rdx, 3
        jae 29f
        xor     edx, edx
        jmp 31f
13:
        xor     edx, edx
        neg     rdx
        neg     rdx
        jmp 4b
14:
        xor     edx, edx
        jmp 4b
15:
        lea     r14, [rax + 1]
        mov     r15, rbx
        and     r15, -4
        xor     edx, edx
16:
        movzx   ebp, byte ptr [r11]
        movzx   r12d, byte ptr [r11 + 1]
        add     bpl, -48
        movsx   r13, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [2*rdx]
        add     rdx, r13
        add     r12b, -48
        movsx   r12, r12b
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        movzx   ebp, byte ptr [r11 + 2]
        add     bpl, -48
        movsx   r12, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        movzx   ebp, byte ptr [r11 + 3]
        add     bpl, -48
        movsx   r12, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        add     r11, 4
        add     r15, -4
        jne 16b
        test    r14b, 3
        je 19f
17:
        and     ebx, 3
        xor     r14d, r14d
18:
        movzx   ebp, byte ptr [r11 + r14]
        add     bpl, -48
        movsx   r15, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r15 + 2*rdx]
        inc     r14
        cmp     rbx, r14
        jne 18b
19:
        neg     rdx
        neg     rdx
        jmp 4b
20:
        lea     r14, [rax + 1]
        mov     r15, rbx
        and     r15, -4
        xor     edx, edx
21:
        movzx   ebp, byte ptr [r11]
        movzx   r12d, byte ptr [r11 + 1]
        add     bpl, -48
        movsx   r13, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [2*rdx]
        add     rdx, r13
        add     r12b, -48
        movsx   r12, r12b
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        movzx   ebp, byte ptr [r11 + 2]
        add     bpl, -48
        movsx   r12, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        movzx   ebp, byte ptr [r11 + 3]
        add     bpl, -48
        movsx   r12, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        add     r11, 4
        add     r15, -4
        jne 21b
        test    r14b, 3
        je 28f
22:
        and     ebx, 3
        xor     r14d, r14d
23:
        movzx   ebp, byte ptr [r11 + r14]
        add     bpl, -48
        movsx   r15, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r15 + 2*rdx]
        inc     r14
        cmp     rbx, r14
        jne 23b
        jmp 28f
24:
        lea     r11, [rax + 2]
        mov     rbx, r14
        and     rbx, -4
        xor     edx, edx
25:
        movzx   ebp, byte ptr [r15]
        movzx   r12d, byte ptr [r15 + 1]
        add     bpl, -48
        movsx   r13, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [2*rdx]
        add     rdx, r13
        add     r12b, -48
        movsx   r12, r12b
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        movzx   ebp, byte ptr [r15 + 2]
        add     bpl, -48
        movsx   r12, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        movzx   ebp, byte ptr [r15 + 3]
        add     bpl, -48
        movsx   r12, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        add     r15, 4
        add     rbx, -4
        jne 25b
        test    r11b, 3
        je 28f
26:
        and     r14d, 3
        xor     r11d, r11d
27:
        movzx   ebx, byte ptr [r15 + r11]
        add     bl, -48
        movsx   rbx, bl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [rbx + 2*rdx]
        inc     r11
        cmp     r14, r11
        jne 27b
28:
        neg     rdx
        jmp 4b
29:
        lea     r11, [rax + 2]
        mov     rbx, r14
        and     rbx, -4
        xor     edx, edx
30:
        movzx   ebp, byte ptr [r15]
        movzx   r12d, byte ptr [r15 + 1]
        add     bpl, -48
        movsx   r13, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [2*rdx]
        add     rdx, r13
        add     r12b, -48
        movsx   r12, r12b
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        movzx   ebp, byte ptr [r15 + 2]
        add     bpl, -48
        movsx   r12, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        movzx   ebp, byte ptr [r15 + 3]
        add     bpl, -48
        movsx   r12, bpl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [r12 + 2*rdx]
        add     r15, 4
        add     rbx, -4
        jne 30b
        test    r11b, 3
        je 4b
31:
        and     r14d, 3
        xor     r11d, r11d
32:
        movzx   ebx, byte ptr [r15 + r11]
        add     bl, -48
        movsx   rbx, bl
        lea     rdx, [rdx + 4*rdx]
        lea     rdx, [rbx + 2*rdx]
        inc     r11
        cmp     r14, r11
        jne 32b
        jmp 4b
33:
        pop     rbx
        pop     r12
        pop     r13
        pop     r14
        pop     r15
        pop     rbp
        mov     rax, rcx
        jmp 86f

35:
        mov     rax, rdi
        sub     rax, rsi
        jae 36f
        sub     rsi, rdi
        mov     ecx, esi
        and     ecx, 3
        cmp     rax, -4
        jbe 37f
        xor     eax, eax
        jmp 39f
36:
        xor     eax, eax
        ret
37:
        and     rsi, -4
        xor     eax, eax
38:
        movzx   edx, byte ptr [rdi]
        movzx   r8d, byte ptr [rdi + 1]
        add     dl, -48
        movsx   rdx, dl
        lea     rax, [rax + 4*rax]
        lea     rax, [rdx + 2*rax]
        add     r8b, -48
        movsx   rdx, r8b
        lea     rax, [rax + 4*rax]
        lea     rax, [rdx + 2*rax]
        movzx   edx, byte ptr [rdi + 2]
        add     dl, -48
        movsx   rdx, dl
        lea     rax, [rax + 4*rax]
        lea     rax, [rdx + 2*rax]
        movzx   edx, byte ptr [rdi + 3]
        add     dl, -48
        movsx   rdx, dl
        lea     rax, [rax + 4*rax]
        lea     rax, [rdx + 2*rax]
        add     rdi, 4
        add     rsi, -4
        jne 38b
        test    rcx, rcx
        je 41f
39:
        xor     edx, edx
40:
        movzx   esi, byte ptr [rdi + rdx]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        inc     rdx
        cmp     rcx, rdx
        jne 40b
41:
        ret

42:
        cmp     byte ptr [rdi], 45
        jne 43f
        lea     rcx, [rdi + 1]
        cmp     rcx, rsi
        jae 44f
        mov     r8, rdi
        not     r8
        add     r8, rsi
        sub     rsi, rdi
        add     rsi, -2
        mov     edx, r8d
        and     edx, 3
        cmp     rsi, 3
        jae 46f
        xor     eax, eax
        jmp 48f
43:
        mov     rax, rdi
        sub     rax, rsi
        jae 45f
        sub     rsi, rdi
        mov     ecx, esi
        and     ecx, 3
        cmp     rax, -4
        jbe 51f
        xor     eax, eax
        jmp 53f
44:
        xor     eax, eax
        neg     rax
        ret
45:
        xor     eax, eax
        ret
46:
        and     r8, -4
        xor     eax, eax
47:
        movzx   esi, byte ptr [rcx]
        movzx   edi, byte ptr [rcx + 1]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     dil, -48
        movsx   rsi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 2]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 3]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     rcx, 4
        add     r8, -4
        jne 47b
        test    rdx, rdx
        je 50f
48:
        xor     esi, esi
49:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 49b
50:
        neg     rax
        ret
51:
        and     rsi, -4
        xor     eax, eax
52:
        movzx   edx, byte ptr [rdi]
        movzx   r8d, byte ptr [rdi + 1]
        add     dl, -48
        movsx   rdx, dl
        lea     rax, [rax + 4*rax]
        lea     rax, [rdx + 2*rax]
        add     r8b, -48
        movsx   rdx, r8b
        lea     rax, [rax + 4*rax]
        lea     rax, [rdx + 2*rax]
        movzx   edx, byte ptr [rdi + 2]
        add     dl, -48
        movsx   rdx, dl
        lea     rax, [rax + 4*rax]
        lea     rax, [rdx + 2*rax]
        movzx   edx, byte ptr [rdi + 3]
        add     dl, -48
        movsx   rdx, dl
        lea     rax, [rax + 4*rax]
        lea     rax, [rdx + 2*rax]
        add     rdi, 4
        add     rsi, -4
        jne 52b
        test    rcx, rcx
        je 55f
53:
        xor     edx, edx
54:
        movzx   esi, byte ptr [rdi + rdx]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        inc     rdx
        cmp     rcx, rdx
        jne 54b
55:
        ret

56:
        lea     rcx, [rdi + 1]
        movzx   eax, byte ptr [rdi + 1]
        cmp     byte ptr [rdi], 76
        jne 57f
        cmp     al, 45
        jne 58f
        lea     rcx, [rdi + 2]
        cmp     rcx, rsi
        jae 61f
        sub     rsi, rdi
        lea     rdi, [rsi - 2]
        add     rsi, -3
        mov     edx, edi
        and     edx, 3
        cmp     rsi, 3
        jae 63f
        xor     eax, eax
        jmp 65f
57:
        cmp     al, 45
        jne 59f
        lea     rcx, [rdi + 2]
        cmp     rcx, rsi
        jae 60f
        sub     rsi, rdi
        lea     rdi, [rsi - 2]
        add     rsi, -3
        mov     edx, edi
        and     edx, 3
        cmp     rsi, 3
        jae 68f
        xor     eax, eax
        jmp 70f
58:
        cmp     rcx, rsi
        jae 60f
        mov     r8, rdi
        not     r8
        add     r8, rsi
        sub     rsi, rdi
        add     rsi, -2
        mov     edx, r8d
        and     edx, 3
        cmp     rsi, 3
        jae 73f
        xor     eax, eax
        jmp 75f
59:
        cmp     rcx, rsi
        jae 62f
        mov     r8, rdi
        not     r8
        add     r8, rsi
        sub     rsi, rdi
        add     rsi, -2
        mov     edx, r8d
        and     edx, 3
        cmp     rsi, 3
        jae 78f
        xor     eax, eax
        jmp 80f
60:
        xor     eax, eax
        neg     rax
        ret
61:
        xor     eax, eax
        neg     rax
        neg     rax
        ret
62:
        xor     eax, eax
        ret
63:
        and     rdi, -4
        xor     eax, eax
64:
        movzx   esi, byte ptr [rcx]
        movzx   r8d, byte ptr [rcx + 1]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     r8b, -48
        movsx   rsi, r8b
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 2]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 3]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     rcx, 4
        add     rdi, -4
        jne 64b
        test    rdx, rdx
        je 67f
65:
        xor     esi, esi
66:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 66b
67:
        neg     rax
        neg     rax
        ret
68:
        and     rdi, -4
        xor     eax, eax
69:
        movzx   esi, byte ptr [rcx]
        movzx   r8d, byte ptr [rcx + 1]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     r8b, -48
        movsx   rsi, r8b
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 2]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 3]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     rcx, 4
        add     rdi, -4
        jne 69b
        test    rdx, rdx
        je 72f
70:
        xor     esi, esi
71:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 71b
72:
        neg     rax
        ret
73:
        and     r8, -4
        xor     eax, eax
74:
        movzx   esi, byte ptr [rcx]
        movzx   edi, byte ptr [rcx + 1]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     dil, -48
        movsx   rsi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 2]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 3]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     rcx, 4
        add     r8, -4
        jne 74b
        test    rdx, rdx
        je 77f
75:
        xor     esi, esi
76:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 76b
77:
        neg     rax
        ret
78:
        and     r8, -4
        xor     eax, eax
79:
        movzx   esi, byte ptr [rcx]
        movzx   edi, byte ptr [rcx + 1]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     dil, -48
        movsx   rsi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 2]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        movzx   esi, byte ptr [rcx + 3]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        add     rcx, 4
        add     r8, -4
        jne 79b
        test    rdx, rdx
        je 82f
80:
        xor     esi, esi
81:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 81b
82:
        ret
86:
        "#, in("rsi") input.len(), in("rdi") input.as_ptr(), out("rax") res,
        clobber_abi("C"),
        )
    }
    res
}

fn main() {
    println!(
        "{}",
        run(r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#
            .as_bytes())
    )
}
