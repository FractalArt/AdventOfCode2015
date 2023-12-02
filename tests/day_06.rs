use aoc2015::{self, read_data};

#[test]
fn test_day_06() {
    let data = read_data::<String, _>("data/day_06.txt").unwrap();
    let task_1 = aoc2015::day_06::day_06_1(&data);
    assert_eq!(task_1, 543903);
    let task_2 = aoc2015::day_06::day_06_2(&data);
    assert_eq!(task_2, 14687245);
}
