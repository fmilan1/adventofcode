use std::collections::HashMap;

fn program(prog: &Vec<usize>, registers: &mut HashMap<char, usize>) -> String {
    let mut instruction_pointer: usize = 0;
    let mut output: Vec<usize> = Vec::new();
    while instruction_pointer < prog.len() {
        let opcode = prog[instruction_pointer];
        let mut combo_op = prog[instruction_pointer + 1];
        let literal_op = prog[instruction_pointer + 1];
        match combo_op {
            4 => combo_op = registers[&'A'],
            5 => combo_op = registers[&'B'],
            6 => combo_op = registers[&'C'],
            _ => ()
        }
        match opcode {
            0 => {
                let numerator = registers[&'A'];
                let denominator = usize::pow(2, combo_op as u32);
                registers.insert('A', &numerator / &denominator);
                // dbg!(numerator, denominator, numerator / denominator, registers[&'A']);
                instruction_pointer += 2;
            },
            1 => {
                registers.insert('B', registers[&'B'] ^ literal_op);
                instruction_pointer += 2;
            },
            2 => {
                registers.insert('B', combo_op % 8);
                instruction_pointer += 2;
            },
            3 => {
                if registers[&'A'] != 0 { instruction_pointer = literal_op; }
                else { instruction_pointer += 2; }
            },
            4 => {
                registers.insert('B', registers[&'B'] ^ registers[&'C']);
                instruction_pointer += 2;
            },
            5 => {
                output.push(combo_op % 8);
                instruction_pointer += 2;
            },
            6 => {
                let numerator = registers[&'A'];
                let denominator = usize::pow(2, combo_op as u32);
                registers.insert('B', &numerator / &denominator);
                instruction_pointer += 2;
            },
            7 => {
                let numerator = registers[&'A'];
                let denominator = usize::pow(2, combo_op as u32);
                registers.insert('C', &numerator / &denominator);
                instruction_pointer += 2;
            },
            _ => ()
        }
    }
    output.iter().map(|o| o.to_string()).collect::<Vec<String>>().join(",")
}

fn main() {
    let file = include_str!("../input2.txt");
    let prog: Vec<usize> = file.split("\n\n").nth(1).unwrap().trim().split(" ").nth(1).unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    let mut registers: HashMap<char, usize> = HashMap::new();
    for line in file.split("\n\n").nth(0).unwrap().lines() {
        let l = line.replace(":", "");
        let reg = l.split(" ").nth(1).unwrap().to_string().chars().nth(0).unwrap();
        let val: usize = l.split(" ").nth(2).unwrap().parse().unwrap();
        registers.insert(reg, val);
    }
    println!("{}", program(&prog, &mut registers));
}
