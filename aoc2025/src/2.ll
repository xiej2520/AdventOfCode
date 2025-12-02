; llc -version
; LLVM (http://llvm.org/):
;   LLVM version 21.1.2
;   Optimized build.
;   Default target: x86_64-unknown-linux-gnu
;   Host CPU: znver4

%ParseRangeRes = type { i64, i64, ptr } ; first, second, next start
; Input ends with a '\n'
; MUST BE VALID RANGE IN buf[start:]
define %ParseRangeRes @parse_range(ptr %buf, ptr %start) {
  %dash_ptr.p = alloca ptr
  store ptr %start, ptr %dash_ptr.p
  
  br label %loop_dash
  
loop_dash:
  %dash_ptr.0 = load ptr, ptr %dash_ptr.p
  %dash_ptr.1 = getelementptr i8, ptr %dash_ptr.0, i64 1
  store ptr %dash_ptr.1, ptr %dash_ptr.p
  br label %loop_dash_cond
loop_dash_cond: ; while dash != '-'
  %dash_ptr_ch = load i8, ptr %dash_ptr.1
  %dash_ptr_ch_eq_dash = icmp eq i8 %dash_ptr_ch, 45
  br i1 %dash_ptr_ch_eq_dash, label %parse_first, label %loop_dash

parse_first:
  %first = call i64 @atoi(ptr %start, ptr %dash_ptr.1)

  %end_ptr.p = alloca ptr
  store ptr %dash_ptr.0, ptr %end_ptr.p

  br label %loop_comma_or_nl
loop_comma_or_nl:
  %end_ptr.0 = load ptr, ptr %end_ptr.p
  %end_ptr.1 = getelementptr i8, ptr %end_ptr.0, i64 1
  store ptr %end_ptr.1, ptr %end_ptr.p
  br label %loop_comma_or_nl_cond

loop_comma_or_nl_cond:
  %end_ptr_ch = load i8, ptr %end_ptr.1
  %end_ptr_ch_eq_nl = icmp eq i8 %end_ptr_ch, 10
  br i1 %end_ptr_ch_eq_nl, label %parse_second, label %loop_comma_cond

loop_comma_cond:
  %end_ptr_ch_eq_comma = icmp eq i8 %end_ptr_ch, 44
  br i1 %end_ptr_ch_eq_comma, label %parse_second, label %loop_comma_or_nl

parse_second:
  %dash_plus_1 = getelementptr i8, ptr %dash_ptr.1, i64 1
  %second = call i64 @atoi(ptr %dash_plus_1, ptr %end_ptr.1)
  
  %res.p = alloca %ParseRangeRes
  %res.0 = load %ParseRangeRes, ptr %res.p
  %res.1 = insertvalue %ParseRangeRes %res.0, i64 %first, 0
  %res.2 = insertvalue %ParseRangeRes %res.1, i64 %second, 1
  %next_start = getelementptr i8, ptr %end_ptr.1, i64 1
  %res.3 = insertvalue %ParseRangeRes %res.2, ptr %next_start, 2
  ret %ParseRangeRes %res.3
}



define i32 @_start() {
  %buf = alloca [1000000 x i8]
  %bytes_read = call i64 @read_stdin(ptr %buf, i64 1000000)
  %buf_end = getelementptr i8, ptr %buf, i64 %bytes_read

  %res.p = alloca i64
  store i64 0, ptr %res.p
  %res2.p = alloca i64
  store i64 0, ptr %res.p
  
  %end = getelementptr i8, ptr %buf, i64 %bytes_read

  %start.p = alloca ptr
  store ptr %buf, ptr %start.p
  %next.p = alloca ptr
  store ptr %buf, ptr %next.p
  
  br label %loop_cond
loop_cond: ; while start < end
  %start.0 = load ptr, ptr %start.p
  %start_lt_end = icmp ult ptr %start.0, %end
  br i1 %start_lt_end, label %loop, label %loop_end
loop:

  %range = call %ParseRangeRes @parse_range(ptr %buf, ptr %start.0, ptr %buf_end)
  %first = extractvalue %ParseRangeRes %range, 0
  %second = extractvalue %ParseRangeRes %range, 1
  %next_start = extractvalue %ParseRangeRes %range, 2
  store ptr %next_start, ptr %start.p
  
  call void @dbg_i64(i64 %first)
  call void @dbg_i64(i64 %second)
  
  br label %loop_cond
loop_end:

  call void @exit(i32 0)
  ret i32 0
}
