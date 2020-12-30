use anyhow::{anyhow, Result};
use input_file::Input;

/// An individual instruction is represented here.
#[derive(Clone, Debug)]
struct Instruction {
    op: String,
    count: i32,
}

impl Instruction {
    /// Returns a new instruction.
    fn new(op: String, count: i32) -> Self {
        Self { op, count }
    }

    /// Takes an iterable and returns a Vector of instructions.
    pub fn from_iter<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Vec<Instruction>> {
        lines
            .map(|line| {
                let mut raw_op = line.split(' ');
                let op = raw_op.next().ok_or_else(|| anyhow!("failed to get op"))?;
                let count = raw_op
                    .next()
                    .ok_or_else(|| anyhow!("failed to get count"))?
                    .parse::<i32>()?;
                Ok(Instruction::new(op.to_string(), count))
            })
            .collect()
    }
}

/// AOC8
fn main() -> Result<()> {
    let instructions = Instruction::from_iter(Input::from_args()?.strings("\n"))?;
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
    Ok(())
}

/// Returns where the boot sequence looped and whether it exited cleanly.
fn find_loop_in_boot(instructions: &[Instruction]) -> (i32, bool) {
    let mut acc: i32 = 0;
    let mut node: usize = 0;
    let mut history: Vec<usize> = Vec::new();

    while !history.iter().any(|n| n == &node) {
        let instruction = &instructions[node];
        match instruction.op.as_str() {
            "acc" => {
                history.push(node);
                acc += instruction.count;
                node += 1;
            }
            "jmp" => {
                history.push(node);
                node = (node as i32 + instruction.count) as usize;
            }
            _ => {
                history.push(node);
                node += 1;
            }
        }
        if node == instructions.len() {
            return (acc, true);
        }
        if node > instructions.len() {
            return (acc, false);
        }
    }

    (acc, false)
}

/// Returns the instruction it repaired and whether it exited cleanly.
fn fix_loop_in_boot(instructions: &[Instruction]) -> (i32, bool) {
    let mut modifiable_instructions: Vec<Instruction> = instructions.to_vec();
    let mut result: (i32, bool) = (0, false);
    for (i, instruction) in instructions.iter().enumerate() {
        if instruction.op == "nop" && instruction.count != 0 {
            modifiable_instructions[i].op = "jmp".to_string();
            result = find_loop_in_boot(&modifiable_instructions[..]);
            modifiable_instructions[i].op = "nop".to_string();
        } else if instruction.op == "jmp" {
            modifiable_instructions[i].op = "nop".to_string();
            result = find_loop_in_boot(&modifiable_instructions[..]);
            modifiable_instructions[i].op = "jmp".to_string();
        }
        if result.1 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{find_loop_in_boot, fix_loop_in_boot, Instruction};

    #[test]
    fn test_finding_loop() {
        assert_eq!(find_loop_in_boot(&get_instructions()), (5, false));
    }

    #[test]
    fn test_fixing_loop() {
        assert_eq!(fix_loop_in_boot(&get_instructions()), (8, true));
    }

    /// Returns test data.
    fn get_instructions() -> Vec<Instruction> {
        vec![
            Instruction {
                count: 0,
                op: "nop".to_string(),
            },
            Instruction {
                count: 1,
                op: "acc".to_string(),
            },
            Instruction {
                count: 4,
                op: "jmp".to_string(),
            },
            Instruction {
                count: 3,
                op: "acc".to_string(),
            },
            Instruction {
                count: -3,
                op: "jmp".to_string(),
            },
            Instruction {
                count: -99,
                op: "acc".to_string(),
            },
            Instruction {
                count: 1,
                op: "acc".to_string(),
            },
            Instruction {
                count: -4,
                op: "jmp".to_string(),
            },
            Instruction {
                count: 6,
                op: "acc".to_string(),
            },
        ]
    }
}
