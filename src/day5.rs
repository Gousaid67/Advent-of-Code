//Actual Decent Code

#[derive(Debug, Clone, Copy)]
pub struct Vec2i
{
    x: i64,
    y: i64
}
impl Default for Vec2i
{
    fn default() -> Self
    {
        Vec2i
        {
            x: 0,
            y: 0
        }
    }
}
impl Vec2i
{
    fn new(ix: i64, iy: i64) -> Self
    {
        Vec2i
        {
            x: ix,
            y: iy
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Line
{
    start: Vec2i,
    end: Vec2i
}
impl Line
{
    fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self 
    {
        Line
        {
            start:Vec2i::new(x1,y1),
            end:Vec2i::new(x2,y2)
        }
    }
}
impl From<&str> for Line
{
    fn from(input : &str) -> Self
    {
        let mut values : Vec<&str> = input.split(' ').collect::<Vec<&str>>();
        values.remove(1);
        let coords : Vec<Vec<i64>> = values.iter()
        .map(|str| str.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|val| val.parse::<i64>().unwrap()).collect()
        )
        .collect::<Vec<Vec<i64>>>();


        Line
        {
            start: Vec2i::new(coords[0][0], coords[0][1]),
            end: Vec2i::new(coords[1][0], coords[1][1])
        }

    }
}


pub struct Map
{
    coordinates : [[i32; 1000]; 1000]
}
impl Default for Map
{
    fn default() -> Self 
    {
        Map
        {
            coordinates: [[0 ; 1000]; 1000]
        }
    }
}
impl Map
{
    fn count_over2spots(&self)-> i64
    {
        let mut res : i64 = 0;
        
        for row in self.coordinates
        {
            for position in row {
                if position >= 2
                {
                    res += 1;
                }
            }
        }


        res
    }
}

pub fn find_lines(fs_stream: String) -> i64
{
    let split_vec : Vec<&str> = fs_stream.split("\n").collect();
    let mut map = Map::default();
    let mut line_vec : Vec<Line> = Vec::new();

    for string in split_vec {
        line_vec.push(Line::from(string));
    }

    let horiz_verti_vec = sort_horizontal_vertical(&line_vec);

    trace_lines(&mut map, &line_vec);

    println!("{}", map.count_over2spots());






    return 0
    
}

fn sort_horizontal_vertical(lines: &Vec<Line>) -> Vec<Line>
{
    
    let mut new_vec: Vec<Line> = Vec::new();
    for line in lines
    {
        if (line.start.x == line.end.x) || (line.start.y == line.end.y)
        {
            new_vec.push(*line);
        }
    }

    

    return new_vec

}

fn trace_lines(map : &mut Map, lines: &Vec<Line>)
{
    for line in lines
    {
        let dx = (line.end.x - line.start.x).signum();
        let dy = (line.end.y - line.start.y).signum();
        let step_count = if dx == 0 {(line.start.y - line.end.y).abs()} else {(line.start.x - line.end.x).abs()};

        let start_x = line.start.x;
        let start_y = line.start.y;
        for step in 0..=step_count {
            let step_x = step * dx;
            let step_y = step * dy;

            map.coordinates[(start_y + step_y) as usize][(start_x + step_x) as usize] += 1;

            
        }



    }
}