; llc -version
; LLVM (http://llvm.org/):
;   LLVM version 21.1.2
;   Optimized build.
;   Default target: x86_64-unknown-linux-gnu
;   Host CPU: znver4

@nl = private unnamed_addr constant [1 x i8] c"\0A", align 1

; syscall args:
; %rdi, %rsi, %rdx, %rcx, %r8, %r9.

; ssize_t write(int fd, const void *buf, size_t count)
define i64 @write_stdout(ptr %str, i64 %len) {
entry:
    ; %rax = 1 (write syscall)

    ; %rdi = 1 (stdout)
    ; %rsi = ptr %str
    ; %edx = %len

    ; $0: =r (output)
    ; $1: r (ptr %str)
    ; $2: r (i64 %len)

    %res = call i64 asm sideeffect
      "mov $$1, %rax;  mov $$1, %rdi;  mov $1, %rsi;  mov $2, %rdx;  syscall;  mov %rax, $0;",
      "=r,r,r,~{rax},~{rdi},~{rsi},~{rdx},~{dirflag},~{fpsr},~{flags}"
      (ptr %str, i64 %len)

    ret i64 %res
}

define void @write_stdout_nl() {
  call void @write_stdout(ptr @nl, i64 1)
  ret void
}

; ssize_t read(int fd, void *buf, size_t count);
define i64 @read_stdin(ptr %buf, i64 %count) {
entry:
    ; %rax = 0 (read syscall)

    ; %rdi = 0 (stdin)
    ; %rsi = ptr %buf
    ; %rdx = %count

    ; $0: =r (output)
    ; $1: r (ptr %str)
    ; $2: r (i64 %len)

    %res = call i64 asm sideeffect
      "mov $$0, %rax;  mov $$0, %rdi;  mov $1, %rsi;  mov $2, %rdx;  syscall;  mov %rax, $0;",
      "=r,r,r,~{rax},~{rdi},~{rsi},~{rdx},~{dirflag},~{fpsr},~{flags}"
      (ptr %buf, i64 %count)

    ret i64 %res
}


define void @exit(i32 %status) {
  call void asm sideeffect
    "movq $$60, %rax; movl $0, %edi; syscall"
    , "r,~{rax}"
    (i32 %status)
  ret void
}

define i64 @atoi_positive(ptr %start, ptr %end) {
  %res = alloca i64
  store i64 0, ptr %res
  
  %cur = alloca ptr
  store ptr %start, ptr %cur

  br label %loop_cond
loop_cond: ; while cur < end
  %cur.0 = load ptr, ptr %cur

  %check = icmp ult ptr %cur.0, %end
  br i1 %check, label %loop, label %loop_end

loop:
  %cur.1 = load ptr, ptr %cur
  %d = load i8, ptr %cur.1
  %diff = sub i8 %d, 48 ; '0' = 48
  %diff_i64 = sext i8 %diff to i64
  
  %res.0 = load i64, ptr %res
  %res_10 = mul i64 %res.0, 10
  %sum = add i64 %res_10, %diff_i64
  store i64 %sum, ptr %res
  
  %cur.2 = load ptr, ptr %cur
  %cur_increment = getelementptr i8, ptr %cur.2, i8 1
  store ptr %cur_increment, ptr %cur
  
  br label %loop_cond

loop_end:
  
  %res.1 = load i64, ptr %res
  ret i64 %res.1
  
}

; inclusive [start, end) exclusive
define i64 @atoi(ptr %start, ptr %end) {
  %start.0 = load i8, ptr %start
  %is_start_negative_sign = icmp eq i8 %start.0, 45 ; '-' = 45
  br i1 %is_start_negative_sign, label %negative, label %positive

negative:
  %start_plus_1 = getelementptr i8, ptr %start, i64 1 ; increment start to skip '-'
  %neg_res = call i64 @atoi_positive(ptr %start_plus_1, ptr %end)
  %res.0 = sub i64 0, %neg_res
  ret i64 %res.0

positive:
  %res.1 = call i64 @atoi_positive(ptr %start, ptr %end)
  ret i64 %res.1
}

define i64 @itoa_non_negative(i64 %n, ptr %buf) {
  %len = alloca i64
  store i64 0, ptr %len

  %n_cur = alloca i64
  store i64 %n, ptr %n_cur
  
  br label %len_loop ; do-while to handle n = 0
  
len_loop:
  %n_cur.1 = load i64, ptr %n_cur
  %n_cur_div = udiv i64 %n_cur.1, 10
  store i64 %n_cur_div, ptr %n_cur
  
  %len.0 = load i64, ptr %len
  %len_plus_1 = add i64 %len.0, 1
  store i64 %len_plus_1, ptr %len
  
  br label %len_loop_cond

len_loop_cond: ; while n_cur > 0
  %n_cur.0 = load i64, ptr %n_cur

  %is_n_cur_gt_0 = icmp ugt i64 %n_cur.0, 0
  br i1 %is_n_cur_gt_0, label %len_loop, label %len_loop_end

len_loop_end:

  %len.1 = load i64, ptr %len
  %len_sub_1 = sub i64 %len.1, 1
  
  ; buf + len - 1 is the least significant digit 
  %cur_offset = getelementptr i8, ptr %buf, i64 %len_sub_1

  %cur = alloca ptr
  store ptr %cur_offset, ptr %cur

  store i64 %n, ptr %n_cur
  
  br label %write_loop_cond
write_loop_cond: ; while cur >= buf
  %cur.0 = load ptr, ptr %cur
  %is_cur_ge_buf = icmp uge ptr %cur.0, %buf

  br i1 %is_cur_ge_buf, label %write_loop, label %write_loop_end

write_loop:
  %n_cur.2 = load i64, ptr %n_cur
  %n_mod_10 = urem i64 %n_cur.2, 10

  %n_mod_10_char.0 = add i64 %n_mod_10, 48; '0' = 48
  %n_mod_10_char.1 = trunc i64 %n_mod_10_char.0 to i8

  %cur.1 = load ptr, ptr %cur
  store i8 %n_mod_10_char.1, ptr %cur.1

  %cur_decrement.1 = getelementptr i8, ptr %cur.1, i8 -1
  store ptr %cur_decrement.1, ptr %cur
  
  %n_div_10 = udiv i64 %n_cur.2, 10
  store i64 %n_div_10, ptr %n_cur

  br label %write_loop_cond

write_loop_end:

  %len.2 = load i64, ptr %len
  ret i64 %len.2
}


define i64 @itoa(i64 %n, ptr %buf) {
  %is_non_neg = icmp sge i64 %n, 0
  br i1 %is_non_neg, label %non_negative, label %negative
  
negative:
  store i8 45, ptr %buf ; '-' = 45

  %bump = getelementptr i8, ptr %buf, i64 1
  %neg_n = sub i64 0, %n

  %res.0 = call i64 @itoa_non_negative(i64 %neg_n, ptr %bump)
  %res_plus_1 = add i64 %res.0, 1
  ret i64 %res_plus_1

non_negative:

  %res.1 = call i64 @itoa_non_negative(i64 %n, ptr %buf)
  ret i64 %res.1
}

define void @dbg_i64(i64 %n) {
  %buf = alloca [21 x i8]
  %bytes_written = call i64 @itoa(i64 %n, ptr %buf)
  call i32 @write_stdout(ptr %buf, i64 %bytes_written)
  call void @write_stdout_nl()
  ret void
}
