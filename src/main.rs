pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

fn main() {
    assert!(day1::part1() == 65912);
    assert!(day1::part2() == 195625);
    assert!(day2::part1() == 13675);
    assert!(day2::part2() == 14184);
    assert!(day3::part1() == 7568);
    assert!(day3::part2() == 2780);
    assert!(day4::part1() == 441);
    assert!(day4::part2() == 861);
    assert!(day5::part1() == "SBPQRSCDF");
    assert!(day5::part2() == "RGLVRCQSB");
    assert!(day6::part1() == 1140);
    assert!(day6::part2() == 3495);
    assert!(day7::part1() == 1581595);
    assert!(day7::part2() == 1544176);
    assert!(day8::part1() == 1798);
    assert!(day8::part2() == 259308);
}
