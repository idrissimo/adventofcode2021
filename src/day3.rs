const INPUT: &str = include_str!("3.txt");

pub fn run(){
   println!("{}", part1(INPUT));
}

// Retourne un array avec le nombre d apparition du '1' dans chaque colonne + le nombre de lignes.
fn count(input: &str)->[i32;13]{
    input.trim()
        .lines()
        .map(|line|line.trim().chars().collect::<Vec<char>>())
        .fold([0;13],|mut acc,x|{
            for (i,v) in x.iter().enumerate(){
                match v {
                    '1' => acc[i] += 1,
                    _ => println!("Ain't special"),

                }
            }
            acc[12] += 1; //line numbers
            acc
        })
}


//return consumption = gamma*epsilon
pub fn part1(input:&str)-> i32 {
   
let count = count(input);

let ones = &count[..12];
let lines = count[12];
  
let mut g = String::new();
let mut e = String::new();

for (i,v) in ones.iter().enumerate() {
    match v {
        x if *x > lines/2 => { g.push('1');e.push('0');}, 
        x if *x < lines/2 => { g.push('0');e.push('1');},
        _ => println!("test"),
    }
}  

let gamma = i32::from_str_radix(&g, 2).unwrap();
let epsilon = i32::from_str_radix(&e, 2).unwrap();
   

gamma*epsilon

}



#[test]
fn test_d3_first_part(){
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

assert_eq!(198, part1(input));


}