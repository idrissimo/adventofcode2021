const INPUT: &str = include_str!("1.txt");

pub fn first_part(input: &str)-> usize{
   let numbers: Vec<i64> = input.trim().lines()
                                .map(|line|line.trim())
                                .map(|line|line.parse::<i64>().unwrap())
                                .collect();

    numbers.windows(2).filter(|win|win[1] > win[0]).count()
}

pub fn second_part(input: &str)-> usize{

   let numbers: Vec<i64> = input.trim()
                                .lines()
                                .map(|line|line.trim())
                                .map(|line|line.parse::<i64>().unwrap())
                                .collect();
    
    let total: Vec<i64> = numbers.windows(3)
                                .map(|numbers|numbers.iter().sum())
                                .collect();
    
    total.windows(2)
                            .filter(|digits|digits[0]<digits[1])
                            .count()


}


pub fn run() {

    println!("Part-1 {}", first_part(INPUT));
    println!("Part-2 {}", second_part(INPUT));
}
    

#[test]
fn test_first_part(){
    //set up 
    let input= "
199
200
208
210
200
207
240
269
260
263";

    assert_eq!(7, first_part(input));

}

#[test]
fn test_second_part(){
    //set up
    let input = "
607     
618 
618 
617
647
716
769
792";

    assert_eq!(5, second_part(input));

}