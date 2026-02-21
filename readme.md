# Проект 5 модуля, использовнани пе инструментов для отладки в Rust

## 1. Репзиторий
 - ветка main: https://github.com/EugenBA/module4-plugin/tree/main - код исходного приложения
 - ветка dev: https://github.com/EugenBA/module4-plugin/tree/dev - код измененного приложения

## 2. Внесенные изменения
1. Исправлен метод с утечкой буфера - реализован тест test_leak_buffer
2. Исправлен метод с использованием указателя после удаления реализован тест test_use_after_free
3. Исправлен метод подсчета не нулевых байтов - реализован тест counts_non_zero_bytes
4. Исправлен метод счетчика в разных потоках (гонка данных) - реализован тест test_concurrency
5. Исправлен метод подсчета положительных средних - существующий тест проходит averages_only_positive
6. Исправлен метод подсчета четных значений на с доустпом за пределы среза, реализованы тесты: sum_even_numbers_boundary, sum_even_numbers_empty, sum_even_numbers_no_even
7. Внесены изменения в функцию slow_fib_broken - использована мемоизация
8. Внесены изменения в функцию slow_dedup_broken - убраны лишнии аллокации


## 3. Использованные комманды и инструменты
1. Запуск инрационных тестов
   cargo test --all
   Отчет приведен в документе [report.md](artifacts/report.md) п. 1.2, 3.1, 
2. Отладка под rust-gdb
   rust-gdb target/debug/deps/integration-d812a20ffb69d6d0
   Отчет приведен в документе [report.md](artifacts/report.md) п. 2.1
3. Запуск MIRI
   cargo +nightly miri test
   Отчет приведен в документе [report.md](artifacts/report.md) п. 2.2, 3.2
4. Запуск Valgrind
   valgrind --leak-check=full cargo test --tests
   Отчет приведен в документе [report.md](artifacts/report.md) п. 3.3, 7.3
   valgrind --tool=massif --time-unit=ms ./target/release/demo
   Артефакты сбора данных аллокаций: artifacts/before_massif.out.25473, artifacts/after_massif.out.25473
5. Запуск санитайзеров
   RUSTFLAGS="-Zsanitizer=address" cargo test --target x86_64-unknown-linux-gnu
   TSAN_OPTIONS="ignore_noninstrumented_modules=1"  RUSTFLAGS="-Zsanitizer=thread -Cunsafe-allow-abi-mismatch=sanitizer" cargo test --target x86_64-unknown-linux-gnu
   Отчет приведен в документе [report.md](artifacts/report.md) п. 3.4, 3.5, 7.4
6. Запуск инструментов оптимизации 
     - построение falmegraph cargo flamegraph --release 
     Артефакты: artifacts/flamegraph_before.svg, artifacts/flamegraph_after.svg
     Наибольшее время от функции main занимает метод broken_app::algo::slow_fib ~60%
     Было использовано мемоизация (исключение повторных расчетов) и убраны не нужные аллокации
     - запуск бэнчей baseline, criterion
       ./scripts/compare.sh, ./scripts/profile.sh
     Артефакты: artifacts/baseline_before.txt, artifacts/baseline_after.txt, artifacts/criterion_before, artifacts/criterion_after 
     

## 4. Реализованы регрессионные тесты
 - test_leak_buffer
 - test_use_after_free
 - counts_non_zero_bytes
 - test_concurrency
 - averages_only_positive
 - sum_even_numbers_empty
 - sum_even_numbers_no_even
 - реализован тесты Miri, Valgrind, санитайзенры ./scripts/test-all.sh