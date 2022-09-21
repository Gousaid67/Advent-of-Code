#[derive(Copy, Clone, Debug)]
pub struct BingoBoard
{
    board: [[i32; 5]; 5],
    selected: [[bool; 5]; 5],
}
impl Default for BingoBoard
{
    fn default() -> Self
    {
        Self
        {
            board : [[0; 5]; 5],
            selected : [[false; 5]; 5]

        }

    }
    
}
impl BingoBoard
{
    fn new(starting_str: usize, str_vec: &Vec<&str>) -> BingoBoard
    {
        let mut bingo : BingoBoard = BingoBoard
        {
            board : [[0; 5]; 5],
            selected : [[false; 5]; 5]

        };

        if str_vec.len() - starting_str < 6
        {
            
        }

            
        let mut i = 0;
        for entry in &str_vec[starting_str..=starting_str + 4]
        {
            let array = &mut bingo.board[i]; 
            for n in 0..5 
            {
                let str_index = n * 3;
                if entry.chars().nth(str_index).unwrap() == ' ' {
                    array[n] = entry[str_index + 1..str_index + 2].parse::<i32>().unwrap();
                }
                else
                {
                    array[n] = entry[str_index..=str_index + 1].parse::<i32>().unwrap();
                }


            }
            i += 1

                
        }




        bingo
    }
    fn search_number(&mut self, number : i32) -> bool
    {
        for n in 0..5
        {
            let array = self.board[n];
            if array.contains(&number)
            {
                let index = array.iter().position(|&x| x == number);
                match index
                {
                    Some(x) =>
                    {
                        self.selected[n][x] = true;

                    }
                    None =>
                    {
                        continue
                    }

                }
                return true

            }
        }

        false

    }
    fn sum_unselected(&self) -> i64
    {
        let mut sum : i64 = 0;
        for y in 0..5
        {
            for x in 0..5
            {
                if !self.selected[y][x]
                {
                    sum += self.board[y][x] as i64;
                }
            }
        }
        sum
    }
    fn check_win(&self) -> bool 
    {
        // is the top left a winning symbol?
        if self.selected[0][0]
        {
            //check vertical first
            let mut potential_win = true;
            for vert in self.selected
            {
                if !vert[0]
                {
                    potential_win = false;
                    break
                }

                
            }
            if potential_win
            {
                return true
            }

            //check horizontal next
            potential_win = true;
            for horiz in self.selected[0] {

                if !horiz
                {
                    potential_win = false;
                    break
                }
                
            }
            if potential_win
            {
                return true
            }


        }
        // check vertically 
        for y in 1..5 
        {
            if self.selected[y][0]
            {
                //check horizontal
                let mut potential_win = true;
                for horiz in self.selected[y]
                {
                    if !horiz
                    {
                        potential_win = false;
                        break
                    }
                }
                if potential_win
                {
                    return true
                }
            }
            
        }
        //check horizontally
        for x in 1..5
        {
            if self.selected[0][x]
            {
                //check vertical
                let mut potential_win = true;
                for vert in self.selected {
                    if !vert[x]
                    {
                        potential_win = false;
                        break
                    } 
                    
                }
                if potential_win
                {
                    return true
                }
            }
        }
        false
    }

}

pub fn handle_bingo(fs_stream: String) -> i64
{
    let stream_vec : Vec<&str> = fs_stream.split("\n").collect();
    let numbers_list : Vec<&str> = stream_vec[0].split(',').collect();


    let mut curr_index = 2;
    let mut bingo_list : Vec<BingoBoard> = Vec::new();
    let mut last_drawn : i32 = 0;
    let mut winning_index : usize = 0;
    let mut winner_chicken_dinner : BingoBoard = BingoBoard::default();

    loop
    {  

        bingo_list.push(BingoBoard::new(curr_index, &stream_vec));

        if curr_index + 6 > stream_vec.len() - 1
        {
            break
        }

        curr_index += 6;
    }

    for num in numbers_list
    {
        println!("\n {:?}", bingo_list);
     
        let mut to_search_list : Vec<usize> = Vec::new();
        let input = num.parse::<i32>().unwrap();
        let mut removed_count = 0;
        for index in 0..bingo_list.len()
        {
            let res = bingo_list[index - removed_count].search_number(input);
            if res
            {
                if bingo_list[index - removed_count].check_win()
                {
                    
                    winner_chicken_dinner = bingo_list[index - removed_count];
                    last_drawn = num.parse::<i32>().unwrap();
                    bingo_list.remove(index - removed_count);
                    removed_count += 1;
                }
            }

        }
        /*
        for potential_winner in to_search_list {
            if bingo_list[potential_winner].check_win()
            {
                winning_index = potential_winner;
                winner_chicken_dinner = bingo_list[winning_index];
                last_drawn = num.parse::<i32>().unwrap();
                bingo_list.remove(winning_index);
                
            }
            
        }
        */



        

    }

   

    let res = winner_chicken_dinner.sum_unselected() * last_drawn as i64;
    println!("Last Drawn: {0}; Summary of unselected: {1}", last_drawn, winner_chicken_dinner.sum_unselected());
    println!("{:?}", winner_chicken_dinner);
    return res

    
}