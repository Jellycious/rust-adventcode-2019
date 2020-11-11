use std::fs;
use std::io;
use std::io::BufRead;

const FILE: &str = "input.txt";

struct Program {
    program: Vec<usize>,
}

impl Program {
    fn new(filepath: String) -> io::Result<Program> {
        let f = fs::File::open(filepath)?;
        let mut reader = io::BufReader::new(f);
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        buf.pop(); // remove EOL

        //convert line to vector of numbers.
        let iter1 = buf.as_str().split(',');
        let p: Vec<usize> = iter1.map(|s| s.parse().unwrap()).collect();

        Ok(Program { program: p })
    }

    fn compute(&self, noun : usize, verb : usize) -> usize {
        //computes program until it halts.
        let mut running = true;
        let mut pc = 0; // program counter
        let mut mem = self.program.clone();

        //set program alarm state
        mem[1] = noun;
        mem[2] = verb;

        while running {
            match mem[pc] {
                1 => {
                    // addition operand
                    let r1 = mem[pc + 1];
                    let v1 = mem[r1];
                    let r2 = mem[pc + 2];
                    let v2 = mem[r2];
                    let rd = mem[pc + 3];
                    mem[rd] = v1 + v2;
                    pc += 4;
                }
                2 => {
                    //multiplication operand
                    let r1 = mem[pc + 1];
                    let v1 = mem[r1];
                    let r2 = mem[pc + 2];
                    let v2 = mem[r2];
                    let rd = mem[pc + 3];
                    mem[rd] = v1 * v2;
                    pc += 4;
                }
                99 => {
                    // halt operand
                    running = false;
                }
                _ => {
                    panic!("Illegal Operand Encountered");
                }
            }
        }
        mem[0]
    }
}

fn main() -> io::Result<()> {
    let p = Program::new(String::from(FILE)).unwrap();
    brute_force_solution(&p);
    Ok(())
}

fn brute_force_solution(program: &Program) {
    for noun in 0..99 {
        for verb in 0..99 {
            let answer = program.compute(noun, verb);
            if answer == 19690720 {
                println!("Found solution, noun: {:}, verb: {:}", noun, verb);
                return
            }
        }
    }
    println!("Did not found find a solution");

}
