const INPUT: &str = include_str!("3.txt");
const LINES: i32 = 1000;

const TEST_INPUT: &str = "00100
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

pub fn run(){
    part1(TEST_INPUT);
}

// Retourne un array avec le nombre d apparition du '1' dans chaque colonne + le nombre de lignes.
fn count(input: &str)->[i32;6]{
    input.trim()
        .lines()
        .map(|line|line.trim().chars().collect::<Vec<char>>())
        .fold([0;6],|mut acc,x|{
            for (i,v) in x.iter().enumerate(){
                match v {
                    '1' => acc[i] += 1,
                    _ => println!("Ain't special"),

                }
                            
            }
            acc[5] += 1; //line numbers
            acc
                        
        })
}



pub fn part1(input:&str)->[i32;5]{
   
    let count = count(input);

    let ones = &count[..5];
    let lines = count[5];
    let mut gama = [0;5];

    for (i,v) in ones.iter().enumerate() {
        match v {
            x if *x > lines/2 => gama[i] = 1,
            x if *x < lines/2 => gama[i] = 0,
            _ => println!("test"),
        }
    }       
    
    //gama = [1,0,1,1,0,]

    gama


    //(gamma,epsilon,consuption)


}



#[test]
fn test_first_part(){
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

//assert_eq!(198, part1(input));


}