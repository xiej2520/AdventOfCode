# notes

[Miguel Young's LLVM intro](https://mcyoung.xyz/2023/08/01/llvm-ir)

```ll
define i32 @square(i32 %x) {
  %1 = mul i32 %x, %x
  ret i32 %1
}

add i32 %a, %b
sub
mul
sdiv
srem
udiv
urem

and
or
xor
shl
ashr
lshr

trunc
zext
sext

select i1 %bool, i64 %res_true, i64 %res_false
br label %goto_label
br i1 %bool, label %label_true, label %label_false

switch i32 %integer_value, label %default [
  i32 0, label %if_zero,
  i32 1, label %if_one,
  ; etc
]

icmp eq i64 %a %b ; eq, ne, ugt, uge, ult, ule, sgt, sge, slt, sle

%ptr_x = alloca i32
store i32 1, ptr %ptr_x ; *x = 1
load %x = load i32, ptr %ptr_x


br label %loop_start
loop_cond:
  %i = phi i32 [0, %start], [%j, loop] ; 0 if jumped from start, %j if jumped from loop
  ; phi can refer to values that are *not* defined in *all* blocks that dominate the current block
  ; %a dominates %b if every path to %b passes through %a (every value in %a is defined)

  %b = icmp eq %i 0
  br i1 %b, label %loop_exit, label %loop
loop:
  %j = add i32 %i, 1
  br label %loop_cond
loop_exit:
ret 0

%MyStruct = type { i32, {[4 x ii8], i64}}
%v = extractvalue %MyStruct %s, 1, 0, 4 ; s.1.0[4]
%s.1 = insertvalue %MyStruct %s, i64 42, 1, 1 ; creates copy with field s.1.1 set to 42

getelementptr %MyStruct, ptr %p, i64 %idx, i32 1, i32 1 ; &p[idx]
getelementptr i8, ptr %buf, i64 %i ; &(buf + i)
```

## Types

- `i1, i32, i64, i(arbitrary bit width)`
- `float`, `double`
- `void`, `ptr` (opaque pointer, typed pointers `i32 *` deprecated)
- `[n x T]`
- `{T1, T2, ...}` struct, `<{...}>` packed struct
- `<n x T>` SIMD vector
- `%i64slice = type { ptr, i64 }`

## GDB

```bash
gdb -tui
(gdb) run < input/1.in
(gdb) tui layout asm
(gdb) tui layout regs
(gdb) set disassembly-flavor intel
(gdb) disas
(gdb) bt
```

inline assembly:
make `ret` from `_start` jump to the end and have it fall off.
do not want to return to who-knows-where.
need to convert labels to local labels (use script)
