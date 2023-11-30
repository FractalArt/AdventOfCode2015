#[test]
fn test_day_01() {
    let data = std::fs::read_to_string("data/day_01.txt").unwrap();
    let task_1 = aoc2015::day_01::day_01_1(data.trim());
    assert_eq!(task_1, 74);
    let task_2 = aoc2015::day_01::day_01_2(&data);
    assert_eq!(task_2, 1795);
}
