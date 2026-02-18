pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
    let acc = values.iter().filter(|&&x| x % 2 == 0).sum::<i64>();
    acc
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    // реализуем без unsafe
    let not_null_count = input.iter().filter(|&&x| x != 0).count();
    not_null_count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let values: Vec<&i64> = values.iter().filter(|&&x| x > 0).collect();
    if values.is_empty() {
        return 0.0;
    }
    let sum: i64 = values.iter().copied().sum();
    sum as f64 / values.len() as f64
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let raw = Box::into_raw(b);
    let val = unsafe {2 * *raw};
    unsafe { drop(Box::from_raw(raw)); }
    val
}
