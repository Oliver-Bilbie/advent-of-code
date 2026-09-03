pub enum PacketData {
    Value(u64),
    SubPackets(Vec<Packet>),
}

pub struct Packet {
    pub version: u8,
    pub type_id: u8,
    pub payload: PacketData,
    len: usize,
}

impl Packet {
    pub fn from_bin(bin: &str) -> Packet {
        let version = u8::from_str_radix(&bin[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&bin[3..6], 2).unwrap();
        let (payload, len) = if type_id == 4 {
            read_literal(bin)
        } else {
            read_operator(bin)
        };
        Packet {
            version,
            type_id,
            payload,
            len,
        }
    }
}

fn read_literal(bin: &str) -> (PacketData, usize) {
    let mut value = 0;
    let mut i = 6;
    loop {
        value = 16 * value + u64::from_str_radix(&bin[i + 1..i + 5], 2).unwrap();
        match bin.as_bytes()[i] as char {
            '0' => {
                break; // that was the final group
            }
            '1' => {
                i += 5; // continue to the next group
            }
            _ => panic!("invalid binary string"),
        };
    }
    (PacketData::Value(value), i + 5)
}

fn read_operator(bin: &str) -> (PacketData, usize) {
    let mut sub_pkts = vec![];
    let mut i;
    match bin.chars().nth(6) {
        Some('0') => {
            let payload_start = 22;
            i = payload_start;
            let total_len = usize::from_str_radix(&bin[7..payload_start], 2).unwrap();
            while i - payload_start < total_len {
                let pkt = Packet::from_bin(&bin[i..]);
                i += pkt.len;
                sub_pkts.push(pkt);
            }
        }
        Some('1') => {
            i = 18;
            let pkt_ct = u64::from_str_radix(&bin[7..i], 2).unwrap();
            for _ in 0..pkt_ct {
                let pkt = Packet::from_bin(&bin[i..]);
                i += pkt.len;
                sub_pkts.push(pkt);
            }
        }
        _ => panic!("invalid binary string"),
    };
    (PacketData::SubPackets(sub_pkts), i)
}
