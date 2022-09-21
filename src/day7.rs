pub fn calc_fuel(in_str : String) -> i64
{
    let mut init_state: Vec<i16> = in_str.split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<i16>().unwrap()).collect::<Vec<i16>>();
    init_state.sort_unstable();
    let range_min = init_state[0];
    let range_max = init_state[init_state.len() - 1];
    let mut fuel_cost : Vec<i64> = Vec::new();

    for n in range_min..range_max
    {
        let mut total_dist : i64 = 0;

        for position in &init_state
        {
            let dist = (n - position).abs() as i64;
            total_dist += sum_natural(dist);
        }

        fuel_cost.push(total_dist);
    }

    fuel_cost.sort_unstable();

    

    fuel_cost[0]

}

fn sum_natural ( scalar : i64 ) -> i64
{
    ((scalar + 1) * scalar) / 2
}