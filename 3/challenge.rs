use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::time::Instant;


type Point = (i64, i64);

#[derive(Debug)]
enum Dir {
    Up(i64),
    Right(i64),
    Down(i64),
    Left(i64),
}

// Parsing
fn parse_line(line : &str) -> Vec<Dir> {
    // Requires that there is no EOL/EOF in line.
    let iter = line.split(',');
    let mut vec = Vec::new();
    for command in iter {
        vec.push(parse_command(command));
    }
    vec
}

fn parse_command(command : &str) -> Dir {
    let mut char_iter = command.chars();
    let c = char_iter.next().unwrap();

    let nums = char_iter.as_str();
    let num = nums.parse::<i64>().expect("Could not parse num");

    match c {
        'U' => {
            Dir::Up(num)
        },
        'R' => {
            Dir::Right(num)
        },
        'D' => {
            Dir::Down(num)
        },
        'L' => {
            Dir::Left(num)
        },
        _ => {
            panic!(format!("Could not parse direction {:?}", c))
        }
    }
}


fn parse_input(filepath : &str) -> io::Result<(Vec<Dir>, Vec<Dir>)> {
    let f = fs::File::open(filepath)?;
    let mut reader = io::BufReader::new(f);

    let mut buf1 = String::new();
    reader.read_line(&mut buf1)?;
    buf1.pop(); // Remove EOL

    let mut buf2 = String::new();
    reader.read_line(&mut buf2)?;
    buf2.pop();

    let descr1 = parse_line(buf1.as_str());    
    let descr2 = parse_line(buf2.as_str());    
    Ok((descr1, descr2))
}
// End of Parsing

fn get_points(dirs : &Vec<Dir>) -> HashSet<Point>{
    let mut y = 0;
    let mut x = 0;
    let mut v = HashSet::new();

    for d in dirs.iter() {
        match d {
            Dir::Up(e) => {
                for _ in 0..*e {
                    y = y - 1;
                    v.insert((x,y));
                }
            }
            Dir::Down(e) => {
                for _ in 0..*e {
                    y = y + 1;
                    v.insert((x,y));
                }
            }
            Dir::Right(e) => {
                for _ in 0..*e {
                    x = x + 1;
                    v.insert((x,y));
                }
            }
            Dir::Left(e) => {
                for _ in 0..*e {
                    x = x - 1;
                    v.insert((x,y));
                }
            }
        }
    }
    return v
}

fn get_points_as_vec(dirs : &Vec<Dir>) -> Vec<Point>{
    let mut y = 0;
    let mut x = 0;
    let mut v = Vec::new();

    for d in dirs.iter() {
        match d {
            Dir::Up(e) => {
                for _ in 0..*e {
                    y = y - 1;
                    v.push((x,y));
                }
            }
            Dir::Down(e) => {
                for _ in 0..*e {
                    y = y + 1;
                    v.push((x,y));
                }
            }
            Dir::Right(e) => {
                for _ in 0..*e {
                    x = x + 1;
                    v.push((x,y));
                }
            }
            Dir::Left(e) => {
                for _ in 0..*e {
                    x = x - 1;
                    v.insert((x,y));
                }
            }
        }
    }
    return v
}

fn manhatten(p: Point) -> i64 {
    let (x, y) = p;
    return x.abs()+y.abs()
}

fn main() {
    let filepath = "input.txt";
    let start = Instant::now();
    let descriptions = parse_input(filepath).unwrap();

    let points1 = get_points(&descriptions.0);
    let points2 = get_points(&descriptions.1);
    let intersect = points1.intersection(&points2);
    let sol = intersect.into_iter().map(|p| manhatten(*p)).min();
    let dur = start.elapsed();
    println!("{:?}", sol);
    println!("Time Elapsed: {:?}", dur);

}

