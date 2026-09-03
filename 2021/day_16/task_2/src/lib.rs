pub mod packet;
use crate::packet::*;
use wasm_bindgen::prelude::*;

fn hex_to_bin(hex: &str) -> String {
    hex.chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' | 'a' => "1010",
            'B' | 'b' => "1011",
            'C' | 'c' => "1100",
            'D' | 'd' => "1101",
            'E' | 'e' => "1110",
            'F' | 'f' => "1111",
            _ => panic!("invalid hexadecimal character"),
        })
        .collect()
}

fn result(input: &str) -> u64 {
    let bin = hex_to_bin(input.trim());
    let head = Packet::from_bin(&bin);
    head.eval()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The BITS transmission evaluates to: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_hex() {
        let hex = "38006F45291200";
        let bin = "00111000000000000110111101000101001010010001001000000000";
        assert_eq!(hex_to_bin(hex), bin);
    }

    #[test]
    fn it_solves_the_example_1() {
        let input = "C200B40A82";
        assert_eq!(result(&input), 3);
    }

    #[test]
    fn it_solves_the_example_2() {
        let input = "04005AC33890";
        assert_eq!(result(&input), 54);
    }

    #[test]
    fn it_solves_the_example_3() {
        let input = "880086C3E88112";
        assert_eq!(result(&input), 7);
    }

    #[test]
    fn it_solves_the_example_4() {
        let input = "CE00C43D881120";
        assert_eq!(result(&input), 9);
    }

    #[test]
    fn it_solves_the_example_5() {
        let input = "D8005AC2A8F0";
        assert_eq!(result(&input), 1);
    }

    #[test]
    fn it_solves_the_example_6() {
        let input = "F600BC2D8F";
        assert_eq!(result(&input), 0);
    }

    #[test]
    fn it_solves_the_example_7() {
        let input = "9C005AC2F8F0";
        assert_eq!(result(&input), 0);
    }

    #[test]
    fn it_solves_the_example_8() {
        let input = "9C0141080250320F1802104A08";
        assert_eq!(result(&input), 1);
    }
}
