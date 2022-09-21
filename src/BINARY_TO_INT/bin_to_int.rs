/*
GOAL: Convert bool array to an integer
beginning of array = least significant digit(right to left).
*/

impl Vec<bool> 

pub fn to_i64(&self) -> Result()
{
    if self.len() > 64
    {
        Err("Vector length above 64!")
    }
    let mut res : i64 = 0;
    let mut exponent = 1;
    for entry in self
    {
        let boolint = i64::from_bool(entry);
        res +=  boolint * 2^exponent;

    }
}