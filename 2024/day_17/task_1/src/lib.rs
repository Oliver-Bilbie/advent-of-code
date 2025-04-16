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
    operand: u32,
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

    fn get_combo_operand(operand: u32, device: &Device) -> u32 {
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

    fn adv(operand: u32, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.register_a =
            (device.register_a as f64 / 2_f64.powi(combo_operand as i32)).trunc() as u32;
        device.instruction_pointer += 1;
    }

    fn bxl(operand: u32, device: &mut Device) {
        device.register_b = device.register_b ^ operand;
        device.instruction_pointer += 1;
    }

    fn bst(operand: u32, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.register_b = combo_operand % 8;
        device.instruction_pointer += 1;
    }

    fn jnz(operand: u32, device: &mut Device) {
        if device.register_a != 0 {
            device.instruction_pointer = usize::try_from(operand).unwrap();
        } else {
            device.instruction_pointer += 1;
        }
    }

    fn bxc(_operand: u32, device: &mut Device) {
        device.register_b = device.register_b ^ device.register_c;
        device.instruction_pointer += 1;
    }

    fn out(operand: u32, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.output.push(combo_operand % 8);
        device.instruction_pointer += 1;
    }

    fn bdv(operand: u32, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.register_b =
            (device.register_a as f64 / 2_f64.powi(combo_operand as i32)).trunc() as u32;
        device.instruction_pointer += 1;
    }

    fn cdv(operand: u32, device: &mut Device) {
        let combo_operand = Operation::get_combo_operand(operand, device);
        device.register_c =
            (device.register_a as f64 / 2_f64.powi(combo_operand as i32)).trunc() as u32;
        device.instruction_pointer += 1;
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Device {
    program: Vec<Operation>,
    register_a: u32,
    register_b: u32,
    register_c: u32,
    instruction_pointer: usize,
    output: Vec<u32>,
}

impl Device {
    fn execute_program(&mut self) {
        while let Some(operation) = self.program.get(self.instruction_pointer) {
            operation.clone().do_operation(self);
        }
    }

    fn format_output(&self) -> String {
        self.output
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn read_input(input: &str) -> Device {
    let input_lines: Vec<&str> = input.lines().collect();

    let (_, register_a) = input_lines[0].split_at(12);
    let (_, register_b) = input_lines[1].split_at(12);
    let (_, register_c) = input_lines[2].split_at(12);
    let register_a = register_a.parse::<u32>().unwrap();
    let register_b = register_b.parse::<u32>().unwrap();
    let register_c = register_c.parse::<u32>().unwrap();

    let (_, program) = input_lines[4].split_at(9);
    let program: Vec<&str> = program.split(",").collect();
    let program = program
        .chunks(2)
        .map(|operation_chars| Operation {
            opcode: Opcode::from_str(*operation_chars.get(0).unwrap()),
            operand: operation_chars.get(1).unwrap().parse::<u32>().unwrap(),
        })
        .collect();

    Device {
        program,
        register_a,
        register_b,
        register_c,
        instruction_pointer: 0,
        output: vec![],
    }
}

pub fn solve(input: &str) -> String {
    let mut device = read_input(&input);
    device.execute_program();
    let output = device.format_output();
    format!("The device output is: {}", output)
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
            output: vec![],
        };
        assert_eq!(actual_input, expected_input);
    }

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "The device output is: 4,6,3,5,6,3,5,2,1,0";
        assert_eq!(actual_solution, expected_solution);
    }
}
