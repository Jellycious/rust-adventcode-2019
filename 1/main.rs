const FILE: &str = "input.txt";

use std::fs;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = fs::File::open(FILE)?;
    let reader = BufReader::new(f);
    let lines_iter = reader.lines();

    let mut required_fuel = 0;

    for line in lines_iter {
        let l = line.unwrap();
        let mass = l.parse::<i64>().unwrap();
        let m = calculate_fuel(mass);
        let m_extra = calculate_fuel_for_fuel(m);
        required_fuel += m + m_extra;
    }

    println!("Total: {}", required_fuel); 

    Ok(())
}

fn calculate_fuel(mass: i64) -> i64 {
    let m: f64 = mass as f64 / 3 as f64;
    let m = m.floor() as i64;
    let m = m - 2;
    m
}

fn calculate_fuel_for_fuel(fuel: i64) -> i64 {
    if fuel == 0 {
        return 0
    }
    let m : f64 = fuel as f64 / 3 as f64;
    let m = m.floor() as i64;
    let m = m - 2;
    if m < 0 {
        return 0
    }else {
        m + calculate_fuel_for_fuel(m)
    }
}
