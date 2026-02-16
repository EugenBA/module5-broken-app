# 1. Ознакомление
broken-app: https://github.com/EugenBA/module5-broken-app.git
reference-app: https://github.com/EugenBA/module5-reference-app.git

## 1.1 Test broken-app:
 - cargo test --all
```text
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: consult the function's documentation for information on how to avoid undefined behavior

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
warning: `broken-app` (lib test) generated 3 warnings (3 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/debug/deps/broken_app-1fcfe3268fbb11df)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/debug/deps/demo-48b0b01bb0836e32)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-f368c4363e590c2a)

running 6 tests
test averages_only_positive ... FAILED
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok

thread 'sums_even_numbers' (3635230) panicked at src/lib.rs:11:29:
unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice

This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
thread caused non-unwinding panic. aborting.
```

## 1.2 Test reference-app:
 - cargo test --all
```text
warning: unused import: `std::sync::Arc`
 --> src/concurrency.rs:2:5
  |
2 | use std::sync::Arc;
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `reference-app` (lib) generated 1 warning (run `cargo fix --lib -p reference-app` to apply 1 suggestion)
warning: `reference-app` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/reference_app-e4b02bcd08386c61)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/debug/deps/demo-1efe159fc66a8cff)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-a5c8257c3b039a1f)

running 7 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok
test race_increment_is_correct ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests reference_app

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Есть тест который не проходит: <b>test averages_only_positive ... FAILED</b>

# 2. Поиск и исправление багов

## 2.1 GDB
```text
rust-gdb target/debug/deps/integration-d812a20ffb69d6d0
GNU gdb (Ubuntu 15.0.50.20240403-0ubuntu1) 15.0.50.20240403-git
Copyright (C) 2024 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from target/debug/deps/integration-d812a20ffb69d6d0...
(gdb) b broken_app::average_positive
Breakpoint 1 at 0x7a607: file src/lib.rs, line 48.
(gdb) r
Starting program: /mnt/ssd_data/RustProject/module5-eo/broken-app/target/debug/deps/integration-d812a20ffb69d6d0 

This GDB supports auto-downloading debuginfo from the following URLs:
  <https://debuginfod.ubuntu.com>
Enable debuginfod for this session? (y or [n]) y
Debuginfod has been enabled.
To make this setting permanent, add 'set debuginfod enabled on' to .gdbinit.
Downloading separate debug info for system-supplied DSO at 0x7ffff7fc3000
[Thread debugging using libthread_db enabled]                                                                                                                                                                                       
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

running 6 tests
[New Thread 0x7ffff7bff6c0 (LWP 3704292)]
[New Thread 0x7fffef7ff6c0 (LWP 3704293)]
[Thread 0x7fffef7ff6c0 (LWP 3704293) exited]
[Switching to Thread 0x7ffff7bff6c0 (LWP 3704292)]

Thread 2 "averages_only_p" hit Breakpoint 1, broken_app::average_positive (values=&[i64](size=3) = {...}) at src/lib.rs:48
48          let sum: i64 = values.iter().sum();
(gdb) p values
$1 = &[i64](size=3) = {-5, 5, 15}
(gdb) D 1
Undefined command: "D".  Try "help".
(gdb) d
Delete all breakpoints, watchpoints, tracepoints, and catchpoints? (y or n) y
(gdb) b src/lib.rs:53
Breakpoint 2 at 0x5555555ce676: file src/lib.rs, line 53.
(gdb) r
The program being debugged has been started already.
Start it from the beginning? (y or n) y
Starting program: /mnt/ssd_data/RustProject/module5-eo/broken-app/target/debug/deps/integration-d812a20ffb69d6d0 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

running 6 tests
[New Thread 0x7ffff7bff6c0 (LWP 3704720)]
[New Thread 0x7ffff79fe6c0 (LWP 3704721)]
[Switching to Thread 0x7ffff7bff6c0 (LWP 3704720)]

Thread 2 "averages_only_p" hit Breakpoint 2, broken_app::average_positive (values=&[i64](size=3) = {...}) at src/lib.rs:53
53      }
(gdb) p sum
No symbol 'sum' in current context
(gdb) d
Delete all breakpoints, watchpoints, tracepoints, and catchpoints? (y or n) y
(gdb) b src/lib.rs:49
Breakpoint 3 at 0x5555555ce62c: file src/lib.rs, line 49.
(gdb) r
The program being debugged has been started already.
Start it from the beginning? (y or n) y
Starting program: /mnt/ssd_data/RustProject/module5-eo/broken-app/target/debug/deps/integration-d812a20ffb69d6d0 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

running 6 tests
[New Thread 0x7ffff7bff6c0 (LWP 3704956)]
[Switching to Thread 0x7ffff7bff6c0 (LWP 3704956)]

Thread 2 "averages_only_p" hit Breakpoint 3, broken_app::average_positive (values=&[i64](size=3) = {...}) at src/lib.rs:49
49          if values.is_empty() {
(gdb) p sum
$2 = 15
(gdb) p values
$3 = &[i64](size=3) = {-5, 5, 15}
(gdb) 
```
## 2.2 MIRI
```text
cargo +nightly miri test
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: consult the function's documentation for information on how to avoid undefined behavior

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/lib.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/broken_app-8710f211e5fc635a)
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: consult the function's documentation for information on how to avoid undefined behavior

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-877f2d04eadb8ffe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 6 tests
test averages_only_positive ... FAILED
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... error: Undefined Behavior: `assume` called with `false`
  --> src/lib.rs:11:22
   |
11 |             let v = *values.get_unchecked(idx);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: this is on thread `sums_even_numbe`
   = note: stack backtrace:
           0: broken_app::sum_even
               at src/lib.rs:11:22: 11:47
           1: sums_even_numbers
               at tests/integration.rs:7:16: 7:31
           2: sums_even_numbers::{closure#0}
               at tests/integration.rs:4:23: 4:23

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

```
## 2.3 Valgrind
```text
valgrind --leak-check=full cargo test --tests
==3642402== Memcheck, a memory error detector
==3642402== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==3642402== Using Valgrind-3.22.0 and LibVEX; rerun with -h for copyright info
==3642402== Command: cargo test --tests
==3642402== 
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: consult the function's documentation for information on how to avoid undefined behavior

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
warning: `broken-app` (lib test) generated 3 warnings (3 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/lib.rs (target/debug/deps/broken_app-1fcfe3268fbb11df)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/debug/deps/demo-48b0b01bb0836e32)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-f368c4363e590c2a)

running 6 tests
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test counts_non_zero_bytes ... ok
test averages_only_positive ... FAILED
thread 'sums_even_numbers' (3642452) panicked at src/lib.rs:11:29:
unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice

This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
thread caused non-unwinding panic. aborting.
```

## 2.4 Sanitizers
```text
RUSTFLAGS="-Zsanitizer=address" cargo test --target x86_64-unknown-linux-gnu
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: consult the function's documentation for information on how to avoid undefined behavior

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

For more information about this error, try `rustc --explain E0133`.
warning: `broken-app` (lib) generated 3 warnings (run `cargo fix --lib -p broken-app` to apply 1 suggestion)
warning: `broken-app` (lib test) generated 3 warnings (3 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/x86_64-unknown-linux-gnu/debug/deps/broken_app-823f9bd8cd006800)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/x86_64-unknown-linux-gnu/debug/deps/demo-41f520e4cd023b42)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/x86_64-unknown-linux-gnu/debug/deps/integration-6383544173c3801d)

running 6 tests

thread 'sums_even_numbers' (3654154) panicked at src/lib.rs:11:29:
unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice

This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
thread caused non-unwinding panic. aborting.
```

## 2.5 Найденные проблемы

### 2.5.1 rust-gdb
 - тест, не проходящий положительно: <b>averages_only_positive</b>
 - входное значение (вектор): <b>p values $3 = &[i64](size=3) = {-5, 5, 15}</b>
 - значение sum: <b>p sum $2 = 15? нет проверки на позитивные значения, среднее не верно расчитывается</b>

### 2.5.2 MIRI
 - dereference of raw pointer is unsafe and requires unsafe block src/lib.rs:60:15 let val = *raw;
 - an unsafe function restricts its caller, but its body is safe by default src/lib.rs:57:1 pub unsafe fn use_after_free() -> i32
 - call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block src/lib.rs:61:10 pub unsafe fn use_after_free() -> i32
 - dereference of raw pointer is unsafe and requires unsafe block src/lib.rs:62:11  val + *raw
 - test averages_only_positive ... FAILED
 - sums_even_numbers ... error: Undefined Behavior: `assume` called with `false` src/lib.rs:11:22 let v = *values.get_unchecked(idx);

### 2.5.3 Valgrind
 - dereference of raw pointer is unsafe and requires unsafe block src/lib.rs:60:15 let val = *raw;
 - an unsafe function restricts its caller, but its body is safe by default src/lib.rs:57:1 pub unsafe fn use_after_free() -> i32
 - call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block src/lib.rs:61:10 drop(Box::from_raw(raw));
 - dereference of raw pointer is unsafe and requires unsafe block src/lib.rs:62:11 val + *raw
 - test averages_only_positive ... FAILED 
 - thread 'sums_even_numbers' (3642452) panicked at src/lib.rs:11:29:

### 2.5.4 Sanitizers
 - dereference of raw pointer is unsafe and requires unsafe block src/lib.rs:60:15 let val = *raw;
 - an unsafe function restricts its caller, but its body is safe by default src/lib.rs:57:1 pub unsafe fn use_after_free() -> i32
 - call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block src/lib.rs:61:10 drop(Box::from_raw(raw));
 - dereference of raw pointer is unsafe and requires unsafe block src/lib.rs:62:11 val + *raw
 - thread 'sums_even_numbers' (3654154) panicked at src/lib.rs:11:29: unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice
 - thread 'main' (3655315) panicked at src/lib.rs:11:29: unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice


# 3. Подтверждение корректности

# 4. Поиск узких мест

# 5. Бенчмарки до оптимизации

# 6. Оптимизация

# 7. Проверка «после»