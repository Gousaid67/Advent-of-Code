/*
Operation: What in the living fuck is this.

 */

use regex::Regex;
//use lazy_static::lazy_static;

pub fn calculate_bit_pattern(in_str : Vec<&str>) -> i64
{
    let bitlength = in_str[0].len();
    let pattern_count = in_str.len();
    let mut bit_diction : Vec<i32> = Vec::with_capacity(bitlength);
    println!("{}", pattern_count);
    for _i in 0..bitlength
    {
        bit_diction.push(0);

    }

    let mut iter = 0;

    while iter < pattern_count
    {

        let current_bit_pattern = in_str[iter].to_string();
        for n in 0..bitlength
        {
            
            let iterated_char = current_bit_pattern.chars().nth(n).unwrap();

            if iterated_char == '1'
            {
                bit_diction[n] += 1;
            }

        }
        iter += 1
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    println!("{:?}", bit_diction);
    for entry in bit_diction
    {
        if entry > (pattern_count / 2) as i32
        {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
        else
        {
            gamma.push_str("0");
            epsilon.push_str("1");
        }

    }

    println!("Gamma: {0}, Epsilon: {1}", gamma, epsilon);

    let gamma_int = i64::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = i64::from_str_radix(&epsilon, 2).unwrap();

    return gamma_int * epsilon_int

}

pub fn calc_oxygen_levels(in_str : String) -> i64
{

    let oxygen_lvl : i64;
    let co2_lvl : i64;
    let mut oxy_list : Vec<&str> = in_str.split("\n").collect();
    let bitlength = oxy_list[0].len();
    let mut sel_bit_count : usize = 0;


    let mut bit_diction_oxy : Vec<i32> = Vec::with_capacity(bitlength);
    for _i in 0..bitlength
    {
        bit_diction_oxy.push(0);

    }
    let mut bit_str_oxy : String = String::new();


    let mut bit_diction_co2 : Vec<i32> = Vec::with_capacity(bitlength);
    for _i in 0..bitlength
    {
        bit_diction_co2.push(0);
    }
    let mut bit_str_co2 : String = String::new();

    while oxy_list.len() != 1
    {

        for n in 0..oxy_list.len() {
            
        
            let iterated_char = oxy_list[n].chars().nth(sel_bit_count).unwrap();

            if iterated_char == '1'
            {
                bit_diction_oxy[sel_bit_count] += 1;
            }

        }
        if bit_diction_oxy[sel_bit_count] >= oxy_list.len() as i32 / 2
        {
            bit_str_oxy.push('1');
        }
        else
        {
            bit_str_oxy.push('0');
        }

        let mut new_list : Vec<&str> = Vec::new();

        for i in 0..oxy_list.len(){
            if oxy_list[i].starts_with(&bit_str_oxy)
            {
                new_list.push(oxy_list[i]);
            }
        }
        oxy_list = new_list;
        sel_bit_count += 1;

    }

    println!("bit pattern: {}", bit_str_oxy);

    println!("Oxygen Level: {}", i64::from_str_radix(oxy_list[0], 2).unwrap());

    oxygen_lvl = i64::from_str_radix(oxy_list[0], 2).unwrap();
    sel_bit_count = 0;

    let mut co2_list : Vec<&str> = in_str.split("\n").collect();


    while co2_list.len() != 1
    {

        println!("{}", co2_list.len());

        for n in 0..co2_list.len() {
            
        
            let iterated_char = co2_list[n].chars().nth(sel_bit_count).unwrap();

            if iterated_char == '1'
            {
                bit_diction_co2[sel_bit_count] += 1;
            }

        }
        println!("{:?}", bit_diction_co2);
        if bit_diction_co2[sel_bit_count] >= co2_list.len() as i32 / 2
        {
            bit_str_co2.push('0');
        }
        else
        {
            bit_str_co2.push('1');
        }

        let mut new_list : Vec<&str> = Vec::new();

        for i in 0..co2_list.len(){
            if co2_list[i].starts_with(&bit_str_co2)
            {
                new_list.push(co2_list[i]);
            }
        }
        co2_list = new_list;
        sel_bit_count += 1;

    }

    println!("co2 bit pattern: {}", &bit_str_co2);
    co2_lvl = i64::from_str_radix(co2_list[0], 2).unwrap();
    println!("CO2 Level: {}", co2_lvl);


    return oxygen_lvl * co2_lvl

}

pub fn build_regex(str_slice : &str, total_count : i64) -> Regex
{
    if str_slice.len() > total_count as usize
    {
        panic!("str_slice is larger than the total count of bits!");
    }
    let mut regex_str : String = String::new();
    regex_str.push_str(r"(^");
    regex_str.push_str(str_slice);
    for _n in 0..total_count - str_slice.len() as i64
    {
        regex_str.push('.');
    } 
    regex_str.push(')');
    return Regex::new(&regex_str).unwrap()

}