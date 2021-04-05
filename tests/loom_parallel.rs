#[test]
#[cfg(loom)]
fn test_parallel_calc() {
    const ANSWER_666: &str = "11956824258286445517629485";
    use daily_programmer_386::{bigint::BigInt, calc_partition_count_parallel};
    loom::model(|| {
        let true_answer: BigInt = ANSWER_666.parse().unwrap();
        let num_threads = 3;
        let parallel_answer = calc_partition_count_parallel(666, num_threads);
        assert_eq!(parallel_answer, true_answer);
    });
}
