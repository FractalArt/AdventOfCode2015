use aoc2015::{self, read_data};

#[test]
fn test_day_02() {
    let data = read_data::<String, _>("data/day_02.txt").unwrap(); 
    let task_1 = aoc2015::day_02::day_02_1(&data);
    assert_eq!(task_1, 1586300);
    let task_2 = aoc2015::day_02::day_02_2(&data);
    assert_eq!(task_2, 3737498);
}
