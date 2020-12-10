use input_file::Input;

#[derive(Clone, Debug)]
struct Instruction {
    verb: String,
    amount: i32,
}

impl Instruction {
    pub fn new(verb: String, amount: i32) -> Result<Self, std::io::Error> {
        Ok(Self { verb, amount })
    }

    pub fn parse(lines: Vec<&str>) -> Vec<Instruction> {
        lines
            .iter()
            .map(|v| {
                let mut raw_op = v.split(' ');
                let verb = raw_op.next().unwrap();
                let amount = (raw_op.next().unwrap()).parse::<i32>().unwrap();
                Instruction::new(verb.to_string(), amount).unwrap()
            })
            .collect()
    }
}

/// AOC8
fn main() {
    let input = Input::from_args().unwrap();
    let instructions = Instruction::parse(input.strings("\n").collect());
    let (is_clean_exit, accumulator) = find_loop_in_boot(&instructions);
    println!(
        "Run 1 accumulator: {}, cleanly exited: {}",
        accumulator, is_clean_exit
    );
    let (is_clean_exit, accumulator) = fix_loop_in_boot(&instructions);
    println!(
        "Fix 1 accumulator: {}, cleanly exited: {}",
        accumulator, is_clean_exit
    );
}

fn find_loop_in_boot(instructions: &[Instruction]) -> (bool, i32) {
    let mut acc: i32 = 0;
    let mut node: usize = 0;
    let mut history: Vec<usize> = Vec::new();

    while !history.iter().any(|n| n == &node) {
        let instruction = &instructions[node];
        match instruction.verb.as_str() {
            "acc" => {
                history.push(node);
                acc += instruction.amount;
                node += 1;
            }
            "jmp" => {
                history.push(node);
                node = (node as i32 + instruction.amount) as usize;
            }
            _ => {
                history.push(node);
                node += 1;
            }
        }
        if node == instructions.len() {
            return (true, acc);
        }
        if node > instructions.len() {
            return (false, acc);
        }
    }

    (false, acc)
}

fn fix_loop_in_boot(instructions: &[Instruction]) -> (bool, i32) {
    let mut modifiable_instructions: Vec<Instruction> = instructions.to_vec();
    let mut result: (bool, i32) = (false, 0);
    for (i, instruction) in instructions.iter().enumerate() {
        if instruction.verb == "nop" && instruction.amount != 0 {
            modifiable_instructions[i].verb = "jmp".to_string();
            result = find_loop_in_boot(&modifiable_instructions[..]);
            modifiable_instructions[i].verb = "nop".to_string();
        } else if instruction.verb == "jmp" {
            modifiable_instructions[i].verb = "nop".to_string();
            result = find_loop_in_boot(&modifiable_instructions[..]);
            modifiable_instructions[i].verb = "jmp".to_string();
        }
        if result.0 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{find_loop_in_boot, fix_loop_in_boot, Instruction};

    fn get_instructions() -> Vec<Instruction> {
        vec![
            Instruction {
                amount: 0,
                verb: "nop".to_string(),
            },
            Instruction {
                amount: 1,
                verb: "acc".to_string(),
            },
            Instruction {
                amount: 4,
                verb: "jmp".to_string(),
            },
            Instruction {
                amount: 3,
                verb: "acc".to_string(),
            },
            Instruction {
                amount: -3,
                verb: "jmp".to_string(),
            },
            Instruction {
                amount: -99,
                verb: "acc".to_string(),
            },
            Instruction {
                amount: 1,
                verb: "acc".to_string(),
            },
            Instruction {
                amount: -4,
                verb: "jmp".to_string(),
            },
            Instruction {
                amount: 6,
                verb: "acc".to_string(),
            },
        ]
    }

    #[test]
    fn test_finding_loop() {
        assert_eq!(find_loop_in_boot(&get_instructions()), (false, 5));
    }

    #[test]
    fn test_fixing_loop() {
        assert_eq!(fix_loop_in_boot(&get_instructions()), (true, 8));
    }
}
