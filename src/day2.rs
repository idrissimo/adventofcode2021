
const INPUT: &str  = include_str!("2.txt");


fn part1(input: &str)-> isize{

    let data: Vec<(&str,isize)> = input.trim()
                    .lines()
                    .map(|line|line.trim())
                    .map(|trimed_line|trimed_line.split_at(trimed_line.find(' ').unwrap()))
                    .map(|(key,val)|(key,val.trim().parse::<isize>().unwrap()))
                    .collect();
        //dbg!(&data);
    let mut down: isize = data.iter()
                                .filter(|dt|dt.0=="down")
                                .fold(0, |acc,x|acc + x.1);
    let mut up: isize = data.iter()
                                .filter(|dt|dt.0=="up")
                                .fold(0,|acc,x|acc + x.1);
    let mut forward : isize = data.iter()
                                    .filter(|dt|dt.0=="forward")
                                    .fold(0,|acc,x|acc + x.1);

    let depth = down - up;
    let result = depth * forward;

    result
        
}

pub fn run(){

    println!("part-1 {}", part1(INPUT));
}

    
    
#[test]
fn test_first_part(){
    let input = "
forward 5
down 5
forward 8
up 3
down 8
forward 2";

assert_eq!(150,part1(input));

}

    


