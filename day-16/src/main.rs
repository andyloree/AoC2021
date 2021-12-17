use std::io::{self, BufRead};
use std::time::{Instant};

fn hexstring_to_bytes(hex: &String) -> Vec<u8> {
    return (0..hex.len()).step_by(2).map(|i| {
        u8::from_str_radix(&hex[i..i + 2], 16).unwrap()
    }).collect();
}



/// Packet - welcome to rust polymorphism
enum Packet {
    Literal(PacketLiteral),
    Op(PacketOperator)
}

/// Common packet header - 6 bits total
struct PacketHeader {
    version: u8,
    type_id: u8
}

/// Literal, variable length value, u64 for part to overflow
struct PacketLiteral {
    header: PacketHeader,
    value: u64
}

/// Operator, collection of subpackets
struct PacketOperator {
    header: PacketHeader,
    subpackets: Vec<Packet>
}

// Our parsed BITS expression tree
struct BITSParser {
    bytes: Vec<u8>,
    bit_idx: usize,
    root_node: Option<Packet>
}

impl BITSParser {
    /// Create an empty expression tree with bytes reference
    fn new(bytes: Vec<u8>) -> Self {
        return BITSParser {
                bytes: bytes,
                bit_idx: 0,
                root_node: None
            }
    }

    /// Parse our expressions, returning a mut self
    fn parse(mut self) -> Self {
        self.root_node = Some(self.parse_packet());
        self
    }

    /// Parses a single packet based upon header.type_id
    fn parse_packet(&mut self) -> Packet {
        let header = self.parse_header();
        match header.type_id {
            4 => return self.parse_literal(header),
            _ => return self.parse_operator(header),
        }
    }

    /// Parses operator packets and their subpackets by len or num packets
    fn parse_operator(&mut self, header: PacketHeader) -> Packet {
        let length_type_id = self.read_bits(1);

        let mut op = PacketOperator { header: header, subpackets: vec!()};

        match length_type_id {
            0 => {  // bit length
                let num_bits = self.read_bits_16(15) as usize;
                let end_bit_idx = self.bit_idx + num_bits;
                while self.bit_idx < end_bit_idx {
                    op.subpackets.push(self.parse_packet());
                }
            }
            1 => {  // num sub packets
                let num_packets = self.read_bits_16(11);
                for _ in 0..num_packets {
                    op.subpackets.push(self.parse_packet());
                }
            },
            _ => unreachable!(),
        }

        let packet = Packet::Op(op);
        return packet;
    }

    /// Parses literal value packet
    fn parse_literal(&mut self, header: PacketHeader) -> Packet {
        let mut value: u64 = 0;
        loop {
            let is_last = self.read_bits(1) == 0;
            value = value << 4;
            value |= self.read_bits(4) as u64;
            if is_last {
                 break;
            }
        }
        let literal = PacketLiteral { header: header, value: value};
        let packet = Packet::Literal(literal);
        return packet;
    }

    /// Consumes 6 bits for version and type_id for header
    fn parse_header(&mut self) -> PacketHeader {
        let version = self.read_bits(3);
        let type_id = self.read_bits(3);
        return PacketHeader {version: version, type_id: type_id };
    }

    /// Bit indexed read across our bytes for lengths up to 8 bits
    fn read_bits(&mut self, len: usize) -> u8 {
        let mut val: u8 = 0;
        for _i in 0..len {
            let byte_idx = self.bit_idx / 8;
            val = val << 1;
            let bit_in_byte_idx = self.bit_idx % 8;
            val |= (self.bytes[byte_idx] & (1 << 7 - bit_in_byte_idx)) >> 7 - bit_in_byte_idx;
            self.bit_idx += 1;
        }
        return val;
    }

    /// Bit indeded read across our bytes for lengths up to 16 bits
    fn read_bits_16(&mut self, len: usize) -> u16 {
        let mut val: u16;
        if len > 8 {
            val = self.read_bits(8) as u16;
            val = val << len - 8;
            val |= self.read_bits(len - 8) as u16;
        }
        else {
            val = self.read_bits(len) as u16;
        }
        return val;
    }

    /// Traverses our expression tree summing packet versions
    fn calc_version_sum(&self) -> u32 {
        match &self.root_node {
            None => 0,
            Some(root_node) => self.packet_version_sum(&root_node)
        }
    }

    /// Sum packet and subpacket versions
    fn packet_version_sum(&self, packet: &Packet) -> u32 {
        match packet {
            Packet::Literal(literal) => return literal.header.version as u32,
            Packet::Op(op) => {
                return op.subpackets.iter().map(|sp| self.packet_version_sum(sp)).sum::<u32>() + op.header.version as u32;
            }
        }
    }

    // Evaluates our entire expression tree
    fn calc_expression(&self) -> u64 {
        match &self.root_node {
            None => 0,
            Some(root_node) => self.packet_expression(&root_node)
        }
    }

    /// Evaluates and returns each expression and subexpression calculation
    fn packet_expression(&self, packet: &Packet) -> u64 {
        match packet {
            Packet::Literal(literal) => return literal.value as u64,
            Packet::Op(op) => {
                match op.header.type_id {
                    0 => op.subpackets.iter().map(|sp| self.packet_expression(sp)).sum(),
                    1 => op.subpackets.iter().map(|sp| self.packet_expression(sp)).product(),
                    2 => op.subpackets.iter().map(|sp| self.packet_expression(sp)).min().unwrap_or(0),
                    3 => op.subpackets.iter().map(|sp| self.packet_expression(sp)).max().unwrap_or(0),
                    5 => (self.packet_expression(&op.subpackets[0]) > self.packet_expression(&op.subpackets[1])) as u64,
                    6 => (self.packet_expression(&op.subpackets[0]) < self.packet_expression(&op.subpackets[1])) as u64,
                    7 => (self.packet_expression(&op.subpackets[0]) == self.packet_expression(&op.subpackets[1])) as u64,
                    _ => unreachable!()
                }
            }
        }
    }
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let line: String = stdin.lock().lines().nth(0).unwrap().unwrap();
    let bytes = hexstring_to_bytes(&line);
    let parser = BITSParser::new(bytes).parse();
        
    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Version sum: {}\r\n", parser.calc_version_sum());

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Expression value: {}", parser.calc_expression());

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}