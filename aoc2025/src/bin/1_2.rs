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
        mov     r9d, 50
        xor     ecx, ecx
        movabs  r10, -6640827866535438581
        movabs  r11, 2951479051793528259
        xor     ebx, ebx
        jmp 5f
3:
        xor     r15d, r15d
        neg     r15
4:
        lea     r8, [r15 + r9]
        mov     rax, r8
        imul    r10
        add     rdx, r8
        mov     rax, rdx
        shr     rax, 63
        sar     rdx, 6
        add     rdx, rax
        mov     rax, rdx
        neg     rax
        cmovs   rax, rdx
        add     rbx, r14
        add     rbx, 4
        test    r9, r9
        setne   dl
        test    r8, r8
        setle   r8b
        and     r8b, dl
        movzx   edx, r8b
        add     rcx, rdx
        add     rcx, rax
        add     r9, r15
        add     r9, 100000000
        mov     rax, r9
        shr     rax, 2
        mul     r11
        shr     rdx, 2
        imul    rax, rdx, 100
        sub     r9, rax
        cmp     rbx, rsi
        jae 33f
5:
        lea     rax, [rdi + rbx]
        mov     r15, -4
        mov     r12, -1
        mov     r13, -2
6:
        mov     r8, r12
        mov     rdx, r13
        lea     r12, [rbx + r15]
        add     r12, 5
        lea     r14, [r15 + 1]
        cmp     r12, rsi
        ja 7f
        lea     r12, [r8 + 1]
        lea     r13, [rdx + 1]
        cmp     byte ptr [rax + r15 + 4], 10
        mov     r15, r14
        jne 6b
7:
        lea     r15, [r14 + rax]
        add     r15, 3
        movzx   ebp, byte ptr [rdi + rbx + 1]
        cmp     byte ptr [rdi + rbx], 76
        lea     r12, [rdi + rbx + 1]
        jne 8f
        cmp     bpl, 45
        jne 9f
        add     rax, 2
        cmp     rax, r15
        jae 13f
        cmp     r14, 3
        jae 15f
        xor     r15d, r15d
        jmp 17f
8:
        cmp     bpl, 45
        jne 12f
        add     rax, 2
        cmp     rax, r15
        jae 3b
        cmp     r14, 3
        jae 20f
        xor     r15d, r15d
        jmp 22f
9:
        cmp     r12, r15
        jae 3b
        lea     rax, [r14 + 1]
        cmp     rax, 3
        jae 24f
        xor     r15d, r15d
        jmp 26f
12:
        cmp     r12, r15
        jae 14f
        lea     rax, [r14 + 1]
        cmp     rax, 3
        jae 29f
        xor     r15d, r15d
        jmp 31f
13:
        xor     r15d, r15d
        neg     r15
        neg     r15
        jmp 4b
14:
        xor     r15d, r15d
        jmp 4b
15:
        lea     r8, [r14 + 1]
        mov     r12, rdx
        and     r12, -4
        xor     r15d, r15d
16:
        movzx   ebp, byte ptr [rax]
        movzx   r13d, byte ptr [rax + 1]
        add     bpl, -48
        movsx   rbp, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, rbp
        add     r13b, -48
        movsx   r13, r13b
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        movzx   ebp, byte ptr [rax + 2]
        add     bpl, -48
        movsx   r13, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        movzx   ebp, byte ptr [rax + 3]
        add     bpl, -48
        movsx   r13, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        add     rax, 4
        add     r12, -4
        jne 16b
        test    r8b, 3
        je 19f
17:
        and     edx, 3
        xor     r8d, r8d
18:
        movzx   ebp, byte ptr [rax + r8]
        add     bpl, -48
        movsx   r12, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [r12 + 2*r15]
        inc     r8
        cmp     rdx, r8
        jne 18b
19:
        neg     r15
        neg     r15
        jmp 4b
20:
        lea     r8, [r14 + 1]
        mov     r12, rdx
        and     r12, -4
        xor     r15d, r15d
21:
        movzx   ebp, byte ptr [rax]
        movzx   r13d, byte ptr [rax + 1]
        add     bpl, -48
        movsx   rbp, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, rbp
        add     r13b, -48
        movsx   r13, r13b
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        movzx   ebp, byte ptr [rax + 2]
        add     bpl, -48
        movsx   r13, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        movzx   ebp, byte ptr [rax + 3]
        add     bpl, -48
        movsx   r13, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        add     rax, 4
        add     r12, -4
        jne 21b
        test    r8b, 3
        je 28f
22:
        and     edx, 3
        xor     r8d, r8d
23:
        movzx   ebp, byte ptr [rax + r8]
        add     bpl, -48
        movsx   r12, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [r12 + 2*r15]
        inc     r8
        cmp     rdx, r8
        jne 23b
        jmp 28f
24:
        lea     rax, [r14 + 2]
        mov     rdx, r8
        and     rdx, -4
        xor     r15d, r15d
25:
        movzx   ebp, byte ptr [r12]
        movzx   r13d, byte ptr [r12 + 1]
        add     bpl, -48
        movsx   rbp, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, rbp
        add     r13b, -48
        movsx   r13, r13b
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        movzx   ebp, byte ptr [r12 + 2]
        add     bpl, -48
        movsx   r13, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        movzx   ebp, byte ptr [r12 + 3]
        add     bpl, -48
        movsx   r13, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        add     r12, 4
        add     rdx, -4
        jne 25b
        test    al, 3
        je 28f
26:
        and     r8d, 3
        xor     eax, eax
27:
        movzx   edx, byte ptr [r12 + rax]
        add     dl, -48
        movsx   rdx, dl
        lea     r15, [r15 + 4*r15]
        lea     r15, [rdx + 2*r15]
        inc     rax
        cmp     r8, rax
        jne 27b
28:
        neg     r15
        jmp 4b
29:
        lea     rax, [r14 + 2]
        mov     rdx, r8
        and     rdx, -4
        xor     r15d, r15d
30:
        movzx   ebp, byte ptr [r12]
        movzx   r13d, byte ptr [r12 + 1]
        add     bpl, -48
        movsx   rbp, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, rbp
        add     r13b, -48
        movsx   r13, r13b
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        movzx   ebp, byte ptr [r12 + 2]
        add     bpl, -48
        movsx   r13, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        movzx   ebp, byte ptr [r12 + 3]
        add     bpl, -48
        movsx   r13, bpl
        lea     r15, [r15 + 4*r15]
        lea     r15, [2*r15]
        add     r15, r13
        add     r12, 4
        add     rdx, -4
        jne 30b
        test    al, 3
        je 4b
31:
        and     r8d, 3
        xor     eax, eax
32:
        movzx   edx, byte ptr [r12 + rax]
        add     dl, -48
        movsx   rdx, dl
        lea     r15, [r15 + 4*r15]
        lea     r15, [rdx + 2*r15]
        inc     rax
        cmp     r8, rax
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
        jmp 82f

34:
        mov     rax, rdi
        sub     rax, rsi
        jae 35f
        sub     rsi, rdi
        mov     ecx, esi
        and     ecx, 3
        cmp     rax, -4
        jbe 36f
        xor     eax, eax
        jmp 38f
35:
        xor     eax, eax
        ret
36:
        and     rsi, -4
        xor     eax, eax
37:
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
        jne 37b
        test    rcx, rcx
        je 40f
38:
        xor     edx, edx
39:
        movzx   esi, byte ptr [rdi + rdx]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        inc     rdx
        cmp     rcx, rdx
        jne 39b
40:
        ret

41:
        cmp     byte ptr [rdi], 45
        jne 42f
        lea     rcx, [rdi + 1]
        cmp     rcx, rsi
        jae 43f
        mov     r8, rdi
        not     r8
        add     r8, rsi
        sub     rsi, rdi
        add     rsi, -2
        mov     edx, r8d
        and     edx, 3
        cmp     rsi, 3
        jae 45f
        xor     eax, eax
        jmp 47f
42:
        mov     rax, rdi
        sub     rax, rsi
        jae 44f
        sub     rsi, rdi
        mov     ecx, esi
        and     ecx, 3
        cmp     rax, -4
        jbe 50f
        xor     eax, eax
        jmp 52f
43:
        xor     eax, eax
        neg     rax
        ret
44:
        xor     eax, eax
        ret
45:
        and     r8, -4
        xor     eax, eax
46:
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
        jne 46b
        test    rdx, rdx
        je 49f
47:
        xor     esi, esi
48:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 48b
49:
        neg     rax
        ret
50:
        and     rsi, -4
        xor     eax, eax
51:
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
        jne 51b
        test    rcx, rcx
        je 54f
52:
        xor     edx, edx
53:
        movzx   esi, byte ptr [rdi + rdx]
        add     sil, -48
        movsx   rsi, sil
        lea     rax, [rax + 4*rax]
        lea     rax, [rsi + 2*rax]
        inc     rdx
        cmp     rcx, rdx
        jne 53b
54:
        ret

55:
        lea     rcx, [rdi + 1]
        movzx   eax, byte ptr [rdi + 1]
        cmp     byte ptr [rdi], 76
        jne 56f
        cmp     al, 45
        jne 57f
        lea     rcx, [rdi + 2]
        cmp     rcx, rsi
        jae 60f
        sub     rsi, rdi
        lea     rdi, [rsi - 2]
        add     rsi, -3
        mov     edx, edi
        and     edx, 3
        cmp     rsi, 3
        jae 62f
        xor     eax, eax
        jmp 64f
56:
        cmp     al, 45
        jne 58f
        lea     rcx, [rdi + 2]
        cmp     rcx, rsi
        jae 59f
        sub     rsi, rdi
        lea     rdi, [rsi - 2]
        add     rsi, -3
        mov     edx, edi
        and     edx, 3
        cmp     rsi, 3
        jae 67f
        xor     eax, eax
        jmp 69f
57:
        cmp     rcx, rsi
        jae 59f
        mov     r8, rdi
        not     r8
        add     r8, rsi
        sub     rsi, rdi
        add     rsi, -2
        mov     edx, r8d
        and     edx, 3
        cmp     rsi, 3
        jae 72f
        xor     eax, eax
        jmp 74f
58:
        cmp     rcx, rsi
        jae 61f
        mov     r8, rdi
        not     r8
        add     r8, rsi
        sub     rsi, rdi
        add     rsi, -2
        mov     edx, r8d
        and     edx, 3
        cmp     rsi, 3
        jae 77f
        xor     eax, eax
        jmp 79f
59:
        xor     eax, eax
        neg     rax
        ret
60:
        xor     eax, eax
        neg     rax
        neg     rax
        ret
61:
        xor     eax, eax
        ret
62:
        and     rdi, -4
        xor     eax, eax
63:
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
        jne 63b
        test    rdx, rdx
        je 66f
64:
        xor     esi, esi
65:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 65b
66:
        neg     rax
        neg     rax
        ret
67:
        and     rdi, -4
        xor     eax, eax
68:
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
        jne 68b
        test    rdx, rdx
        je 71f
69:
        xor     esi, esi
70:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 70b
71:
        neg     rax
        ret
72:
        and     r8, -4
        xor     eax, eax
73:
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
        jne 73b
        test    rdx, rdx
        je 76f
74:
        xor     esi, esi
75:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 75b
76:
        neg     rax
        ret
77:
        and     r8, -4
        xor     eax, eax
78:
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
        jne 78b
        test    rdx, rdx
        je 81f
79:
        xor     esi, esi
80:
        movzx   edi, byte ptr [rcx + rsi]
        add     dil, -48
        movsx   rdi, dil
        lea     rax, [rax + 4*rax]
        lea     rax, [rdi + 2*rax]
        inc     rsi
        cmp     rdx, rsi
        jne 80b
81:
        ret
82:
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
