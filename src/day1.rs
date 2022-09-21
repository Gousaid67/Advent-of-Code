use std::str;

pub fn count_increases_part1(in_str: Vec<i64>) -> i64
{
    let mut counter : i64 = 0;
    let mut previous_val : i64 = -1; //td::str::FromStr::from_str(&in_str[0]).unwrap();
    let mut current_val : i64 = -1; //std::str::FromStr::from_str(&in_str[1]).unwrap();

    for number in in_str 
        {

        if current_val == -1 {
            current_val = number;
            continue;

        }
        previous_val = current_val;
        current_val = number;

        if current_val > previous_val 
        {
            counter += 1;
        }

        
    }
    return counter;
}

pub fn count_increases_part2(in_str: Vec<i64>) -> i64
{
    let mut counter : i64 = 0;
    let mut iter = 0;
    let mut previous_sum = 0;
    let mut current_sum = 0;
    while iter < in_str.len() - 2
    {
        previous_sum = current_sum;
        
        if iter == 0
        {
            current_sum = in_str[iter + 2] + in_str[iter + 1] + in_str[iter];
            iter += 1;
            continue 


        }
        /*
        if iter == 1 //this is also shit, see above
        {
            current_sum = previous_sum + in_str[iter];
            if current_sum > previous_sum
            {
                counter += 1;

            } 
            iter += 1;
            continue
        }
        if iter == in_str.len() 
        {
            current_sum = in_str[in_str.len() - 2] + in_str[in_str.len() - 1];
            if current_sum > previous_sum
            {
                counter += 1;
            
            }
            iter += 1;
            continue

        }
        if iter == in_str.len() + 1
        {
            current_sum = in_str[in_str.len() - 2];
            if current_sum > previous_sum
            {
                counter += 1
            }
            break

        }
        */

        current_sum = in_str[iter + 2] + in_str[iter + 1] + in_str[iter];
        if current_sum > previous_sum 
        {
            counter += 1;
        }
        iter += 1

    }
    return counter;
}
