#[test]
fn test_day_03() {
    let data = std::fs::read_to_string("data/day_03.txt").unwrap();
    let task_1 = aoc2015::day_03::day_03_1(data.trim());
    assert_eq!(task_1, 2572);
    let task_2 = aoc2015::day_03::day_03_2(&data);
    assert_eq!(task_2, 2631);
}
