

pub fn calculate_pos(in_str : Vec<&str>) -> i64
{
    let mut index = 0;
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    let res;
    while index < in_str.len()
    {
        match in_str[index]
        {
            "forward" => {
                x += str::parse::<i64>(in_str[index + 1]).unwrap();
                y += aim * str::parse::<i64>(in_str[index + 1]).unwrap();
            }
            "up" => {
                aim -= str::parse::<i64>(in_str[index + 1]).unwrap();
            }
            "down" => {
                aim += str::parse::<i64>(in_str[index + 1]).unwrap();
            }
            _ => panic!("Unrecognized command! Command: {}", in_str[index])

        }
        index += 2;
    }
    println!("(x : {0}, y : {1})", x, y);
    res = y * x;
    return res;
    
}