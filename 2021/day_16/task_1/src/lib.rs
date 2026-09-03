pub mod packet;
use crate::packet::*;
use std::collections::VecDeque;
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

fn bfs(head: Packet) -> u64 {
    let mut version_sum = 0;
    let mut queue = VecDeque::<Packet>::new();
    queue.push_back(head);
    while let Some(pkt) = queue.pop_front() {
        version_sum += pkt.version as u64;
        match pkt.payload {
            PacketData::Value(_) => {}
            PacketData::SubPackets(sub_pkts) => {
                queue.extend(sub_pkts);
            }
        };
    }
    version_sum
}

fn result(input: &str) -> u64 {
    let bin = hex_to_bin(input.trim());
    let head = Packet::from_bin(&bin);
    bfs(head)
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The sum of version numbers is: {}", result(input));
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
        let input = "8A004A801A8002F478";
        assert_eq!(result(&input), 16);
    }

    #[test]
    fn it_solves_the_example_2() {
        let input = "620080001611562C8802118E34";
        assert_eq!(result(&input), 12);
    }

    #[test]
    fn it_solves_the_example_3() {
        let input = "C0015000016115A2E0802F182340";
        assert_eq!(result(&input), 23);
    }

    #[test]
    fn it_solves_the_example_4() {
        let input = "A0016C880162017C3686B18A3D4780";
        assert_eq!(result(&input), 31);
    }
}
