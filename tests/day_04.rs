#[test]
fn test_day_04() {
    let task_1 = aoc2015::day_04::day_04("bgvyzdsv", 5);
    assert_eq!(task_1, 254575);
    let task_2 = aoc2015::day_04::day_04("bgvyzdsv", 6);
    assert_eq!(task_2, 1038736);
}
