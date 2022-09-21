mod day7;
pub mod vec;
use std::fs;
use std::collections::HashMap;

fn main() {

    /*
    let mut map : HashMap<i32,i32> = HashMap::new();

    map.insert(2, 5);

    println!("{}", map[&2]);

    map.insert(2, 10);

    println!("{}", map[&2]);
    */
    let _args : Vec<String> = std::env::args().collect();
    //println!("{}" , &args[1]);
    let file = fs::read_to_string("day7.in").unwrap();
    let mut res = 0;

    /* Day1
    let str_vec : Vec<&str> = file.split("\n").collect();
    let int_vec : Vec<i64> = str_vec.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    let res = day1::count_increases_part2(int_vec);
    */
    /* Day2
    let order_list : Vec<&str> = file.split(&['\n', ' ']).collect();
    let res = day2::calculate_pos(order_list);
    */
    /* Day3
    let bit_vec : Vec<&str> = file.split("\n").collect();
    let res = day3::calc_oxygen_levels(file);
    */
    /* Day4
    let res = day4::handle_bingo(file);
    */
    /* Day5
    let res = day5::find_lines(file);
    */
    /* Day6
    res = day6::process_life(file);
    */
    /* Day7
    res = day7::calc_fuel(file);
    */

    println!("{}", res);


}