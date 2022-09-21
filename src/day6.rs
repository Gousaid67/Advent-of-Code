pub fn process_life(in_str : String) -> i64
{


    let init_state: Vec<i8> = in_str.split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<i8>().unwrap()).collect::<Vec<i8>>();
    let mut fish_state : [i64 ; 9] = [0 ; 9];
    
    for entry in init_state
    {
        fish_state[entry as usize] += 1;
    }
    
    for n in 0..256
    {
        let mut new_fish_state = fish_state;
        for i in 1..=8 {
            new_fish_state[i - 1 as usize] = fish_state[i];
            
        }
        new_fish_state[8] = fish_state[0];
        new_fish_state[6] += fish_state[0];

        fish_state = new_fish_state;
        //let mut new_state : Vec<i32> = fish_state.clone();
           
    }

    fish_state.iter().sum()
}