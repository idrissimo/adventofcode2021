
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

fn part2(input: &str)-> (i32, i32, i32){

    let data = input.trim()
                    .lines()
                    .map(|line|line.trim())
                    .map(|line|line.split_at(line.find(' ').unwrap()))
                    .map(|(c,v)|(c, v.trim().parse::<i32>().unwrap()))
                    .fold((0,0,0),|acc,cv|{
                        match cv.0 {
                            "up" =>  (acc.0 - cv.1 , acc.1, acc.2),
                            "down" => (acc.0 + cv.1 , acc.1, acc.2),
                            "forward"=> (acc.0, acc.1 + cv.1, acc.2 + acc.0 * cv.1),
                            _ => acc,
                        }
                    });
    
                    return data;

}

pub fn run(){
    let input = "
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    println!("part-1 {}", part1(INPUT));

    let data = part2(INPUT);
    println!("part-2 {}", data.1 * data.2);
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
#[test]
fn test_second_part(){
    let input = "
forward 5
down 5
forward 8
up 3
down 8
forward 2";

assert_eq!((10,15,60),part2(input));

}

    


