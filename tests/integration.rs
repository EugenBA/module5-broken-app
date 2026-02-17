use broken_app::{algo, leak_buffer, normalize, sum_even, use_after_free};
use broken_app::concurrency::Counter;

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize("   Hello   World   "), "helloworld");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    // Ожидается (5 + 15) / 2 = 10, но текущая реализация делит на все элементы.
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn test_use_after_free(){
    let test = 84_i32;
    unsafe { assert_eq!(use_after_free(), test); }
}

#[test]
fn test_leak_buffer(){
    let test: Vec<u8> = vec![0, 0, 1, 2];
    assert_eq!(leak_buffer(test.as_slice()), 2);
}

#[test]
fn test_concurrency()
{
    let counter = Counter::new();
    let a = counter.race_increment(1000, 10);
    let b = counter.read_after_sleep();
    assert!(a.is_some());
    assert!(b.is_some());
    assert_eq!(a.unwrap(), b.unwrap());

}