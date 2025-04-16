#[derive(PartialEq, Clone, Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn from_str(input: &str) -> Opcode {
        match input {
            "0" => Opcode::Adv,
            "1" => Opcode::Bxl,
            "2" => Opcode::Bst,
            "3" => Opcode::Jnz,
            "4" => Opcode::Bxc,
            "5" => Opcode::Out,
            "6" => Opcode::Bdv,
            "7" => Opcode::Cdv,
            _ => panic!("invalid opcode"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Operation {
    opcode: Opcode,
    operand: u64,
}

impl Operation {
    fn do_operation(&self, device: &mut Device) {
        match self.opcode {
            Opcode::Adv => Operation::adv(self.operand, device),
            Opcode::Bxl => Operation::bxl(self.operand, device),
            Opcode::Bst => Operation::bst(self.operand, device),
            Opcode::Jnz => Operation::jnz(self.operand, device),
            Opcode::Bxc => Operation::bxc(self.operand, device),
            Opcode::Out => Operation::out(self.operand, device),
            Opcode::Bdv => Operation::bdv(self.operand, device),
            Opcode::Cdv => Operation::cdv(self.operand, device),
        }
    }

    fn get_combo_operand(operand: u64, device: &Device) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => device.register_a,
            5 => device.register_b,
            6 => device.register_c,
            _ => panic!("invalid operand"),
        }
    }

    fn adv(operand: u64, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.register_a =
            (device.register_a as f64 / 2_f64.powi(combo_operand as i32)).trunc() as u64;
        device.instruction_pointer += 1;
    }

    fn bxl(operand: u64, device: &mut Device) {
        device.register_b = device.register_b ^ operand;
        device.instruction_pointer += 1;
    }

    fn bst(operand: u64, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.register_b = combo_operand % 8;
        device.instruction_pointer += 1;
    }

    fn jnz(operand: u64, device: &mut Device) {
        if device.register_a != 0 {
            device.instruction_pointer = usize::try_from(operand).unwrap();
        } else {
            device.instruction_pointer += 1;
        }
    }

    fn bxc(_operand: u64, device: &mut Device) {
        device.register_b = device.register_b ^ device.register_c;
        device.instruction_pointer += 1;
    }

    fn out(operand: u64, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.output.push(combo_operand % 8);
        device.instruction_pointer += 1;
    }

    fn bdv(operand: u64, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.register_b =
            (device.register_a as f64 / 2_f64.powi(combo_operand as i32)).trunc() as u64;
        device.instruction_pointer += 1;
    }

    fn cdv(operand: u64, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.register_c =
            (device.register_a as f64 / 2_f64.powi(combo_operand as i32)).trunc() as u64;
        device.instruction_pointer += 1;
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Device {
    program: Vec<Operation>,
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
    input: Vec<u64>,
    output: Vec<u64>,
}

impl Device {
    fn execute_program(&mut self) {
        while let Some(operation) = self.program.get(self.instruction_pointer) {
            operation.clone().do_operation(self);
        }
    }

    fn check_output(&self, solved: usize) -> bool {
        // Check whether the last {solved} digits match
        let out_start = self.output.len() - solved;
        let in_start = self.input.len() - solved;
        self.output[out_start..] == self.input[in_start..]
    }
}

fn read_input(input: &str) -> Device {
    let input_lines: Vec<&str> = input.lines().collect();

    let mut input: Vec<u64> = vec![];

    let (_, register_a) = input_lines[0].split_at(12);
    let (_, register_b) = input_lines[1].split_at(12);
    let (_, register_c) = input_lines[2].split_at(12);
    let register_a = register_a.parse::<u64>().unwrap();
    let register_b = register_b.parse::<u64>().unwrap();
    let register_c = register_c.parse::<u64>().unwrap();

    let (_, program) = input_lines[4].split_at(9);
    let program: Vec<&str> = program.split(",").collect();
    let program = program
        .chunks(2)
        .map(|operation_chars| {
            let opcode_input = *operation_chars.get(0).unwrap();
            let operand_input = *operation_chars.get(1).unwrap();
            input.append(&mut vec![
                opcode_input.parse::<u64>().unwrap(),
                operand_input.parse::<u64>().unwrap(),
            ]);
            Operation {
                opcode: Opcode::from_str(opcode_input),
                operand: operand_input.parse::<u64>().unwrap(),
            }
        })
        .collect();

    Device {
        program,
        register_a,
        register_b,
        register_c,
        instruction_pointer: 0,
        input,
        output: vec![],
    }
}

pub fn solve(input: &str) -> String {
    let device = read_input(&input);

    // Find a starting value for register_a which produces a final digit is correct. We then mutliply by
    // 8 and begin incrementing by 1 once again until the final two digits are correct. Repeat this
    // pattern until all digits match.

    let mut start_a = 0;
    let mut solved = 1;

    loop {
        let mut test_device = device.clone();
        test_device.register_a = start_a;
        test_device.execute_program();

        if test_device.check_output(solved) {
            if solved == test_device.input.len() {
                // Exit once all digits match
                break;
            }

            solved += 1;
            start_a *= 8;
        } else {
            start_a += 1
        }
    }

    format!("Initial a: {}", start_a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_input() {
        let test_input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_input = read_input(&test_input);
        let expected_input = Device {
            program: vec![
                Operation {
                    opcode: Opcode::Adv,
                    operand: 1,
                },
                Operation {
                    opcode: Opcode::Out,
                    operand: 4,
                },
                Operation {
                    opcode: Opcode::Jnz,
                    operand: 0,
                },
            ],
            register_a: 729,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            input: vec![0, 1, 5, 4, 3, 0],
            output: vec![],
        };
        assert_eq!(actual_input, expected_input);
    }
}
