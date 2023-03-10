mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
pub mod util;

#[cfg(test)]
extern crate table_test;
pub enum Data {
    Input,
    Test
}
fn main() {
    println!("Hello, world!");
    println!("Day 1:");
    let (ans, dur)= day1::part1(Data::Input);
    println!("  Part 1: {} in {:.2?}", ans, dur);
    let (ans, dur) = day1::part2(Data::Input);
    println!("  Part 2: {} in {:.2?}", ans, dur);

    println!("Day 2:");
    let (ans, dur) = day2::part1(Data::Input);
    println!("  Part 1: {} in {:.2?}", ans, dur);
    let (ans, dur) = day2::part2(Data::Input);
    println!("  Part 2: {} in {:.2?}", ans, dur);

    println!("Day 3:");
    let (ans, dur) = day3::part1(Data::Input);
    println!("  Part 1: {} in {:.2?}", ans, dur);
    let (ans, dur) = day3::part2(Data::Input);
    println!("  Part 2: {} in {:.2?}", ans, dur);

    println!("Day 4:");
    let (ans, dur) = day4::part1(Data::Input);
    println!("  Part 1: {} in {:.2?}", ans, dur);
    let (ans, dur) = day4::part2(Data::Input);
    println!("  Part 2: {} in {:.2?}", ans, dur);

    println!("Day 5:");
    let (ans, dur) = day5::part1(Data::Input);
    println!("  Part 1: {} in {:.2?}", ans, dur);
    let (ans, dur) = day5::part2(Data::Input);
    println!("  Part 2: {} in {:.2?}", ans, dur);

    println!("Day 6:");
    let res = day6::part1(Data::Input);
    if let Ok(res) = res {
        let (ans, dur) = res;
        println!("  Part 1: {} in {:.2?}", ans, dur);
    }
    let res = day6::part2(Data::Input);
    if let Ok((ans, dur)) = res {
        println!("  Part 2: {} in {:.2?}", ans, dur);
    }

    println!("Day 7:");
    let res = day7::part1(Data::Input);
    if let Ok((ans, dur)) = res {
        println!("  Part 1: {} in {:.2?}", ans, dur);
    }
    let res = day7::part2(Data::Input);
    if let Ok((ans, dur)) = res {
        println!("  Part 2: {} in {:.2?}", ans, dur);
    }
}
