use std::{fmt, fs, iter::repeat};

enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
    Invalid(u8),
}

impl From<u8> for Op {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::Equal,
            _ => Self::Invalid(v),
        }
    }
}

impl fmt::Display for Op {
    /// Formats the value using the given formatter.
    ///
    /// * `f`     - Formatter
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sum => write!(f, "+"),
            Self::Product => write!(f, "*"),
            Self::Minimum => write!(f, "min"),
            Self::Maximum => write!(f, "max"),
            Self::GreaterThan => write!(f, ">"),
            Self::LessThan => write!(f, "<"),
            Self::Equal => write!(f, "=="),
            Self::Invalid(v) => write!(f, "invalid({})", v),
        }
    }
}

enum Packet {
    Literal(u8, u64),              // Version, Literal Value.
    Operator(u8, Op, Vec<Packet>), // Version, Type and Sub-packets.
}

impl Packet {
    /// Parses a hex string containing a binary sequence that encodes a
    /// numeric expression and returns the decoded expression tree along
    /// with the next position in the string to begin parsing.
    ///
    /// NOTE: The returned position is relative to start of the given slice.
    ///
    /// * `s` - The hex data.
    fn parse(s: &str) -> (Self, usize) {
        let bin = Self::hex_str_to_bin(s);
        Self::parse_bin(&bin)
    }

    /// Parses a binary string containing a sequence that encodes a
    /// numeric expression and returns the decoded expression tree along
    /// with the next position in the string to begin parsing.
    ///
    /// NOTE: The returned position is relative to start of the given slice.
    ///
    /// * `s` - The binary data.
    fn parse_bin(s: &str) -> (Self, usize) {
        let version = Self::bin_str_to_decimal(&s[0..3]);
        let packet_type = Self::bin_str_to_decimal(&s[3..6]);

        match packet_type {
            4 => Packet::parse_literal(version, s),
            _ => Packet::parse_operator(version, packet_type.into(), s),
        }
    }

    /// Parses a literal value and returns the decoded value along
    /// with the next position in the string to begin parsing.
    ///
    /// NOTE: The returned position is relative to start of the given slice.
    ///
    /// * `s` - The binary data.
    fn parse_literal(version: u8, s: &str) -> (Self, usize) {
        let n = s.len();
        let mut value = String::new();
        let mut i = 6; // Skip version + type ID
        while i < n {
            let is_end = match s.get(i..=i).unwrap() {
                "1" => false,
                "0" => true,
                s => panic!(
                    "Expecting '0' or '1' prefix for literal group. Found '{}'.",
                    s
                ),
            };

            let group = &s[i + 1..i + 5];
            value += group;
            i += 5;

            if is_end {
                // Ignore remaining.
                break;
            }
        }

        let value = u64::from_str_radix(&value, 2).unwrap();
        let next = if i < n { i } else { n };
        (Self::Literal(version, value), next)
    }

    /// Parses an operator and returns the decoded expression tree along
    /// with the next position in the string to begin parsing.
    ///
    /// NOTE: The returned position is relative to start of the given slice.
    ///
    /// * `s` - The binary data.
    fn parse_operator(version: u8, packet_type: u8, s: &str) -> (Self, usize) {
        // Skip version + type ID
        let (sub_packets, next) = match &s[6..=6] {
            "0" => {
                // Next 15 bits give total length in bits of sub-packets.
                let nbits = usize::from_str_radix(&s[7..22], 2).unwrap();
                let end = 22 + nbits; // not-inclusive

                let (packets, mut next) = Self::parse_sub_packets(&s[22..end]);
                next += 22;
                (packets, next)
            }
            "1" => {
                // Next 11 bits give count of sub-packets.
                let count = usize::from_str_radix(&s[7..18], 2).unwrap();
                let (packets, mut next) = Self::parse_sub_packets_by_count(&s[18..], count);
                next += 18;
                (packets, next)
            }
            s => panic!("Expecting '0' or '1' for length type ID. Found '{}'.", s),
        };

        (
            Self::Operator(version, packet_type.into(), sub_packets),
            next,
        )
    }

    /// Parses all sub-packets in a binary sequence and returns a list of decoded
    /// expression trees along with the next position in the string to begin parsing.
    ///
    /// NOTE: The returned position is relative to start of the given slice.
    ///
    /// * `s` - The binary data.
    fn parse_sub_packets(s: &str) -> (Vec<Self>, usize) {
        let mut packets: Vec<Self> = vec![];
        let mut i = 0;
        while i < s.len() {
            let (packet, next) = Self::parse_bin(&s[i..]);
            packets.push(packet);
            i += next;
        }
        (packets, i)
    }

    /// Parses givn count of sub-packets in a binary sequence and returns a list
    /// of decoded expression trees along with the next position in the string
    /// to begin parsing.
    ///
    /// NOTE: The returned position is relative to start of the given slice.
    ///
    /// * `s` - The binary data.
    fn parse_sub_packets_by_count(s: &str, count: usize) -> (Vec<Self>, usize) {
        let mut packets: Vec<Self> = vec![];
        let mut i = 0;
        let mut n = 0;
        while n < count {
            let (packet, next) = Self::parse_bin(&s[i..]);
            packets.push(packet);
            i += next;
            n += 1;
        }
        (packets, i)
    }

    /// Converts a hexadecimal value between 0 - F to its corresponding 4-bit
    /// representation.
    ///
    /// * `c` - The hexadecimal value.
    fn hex_char_to_bin(c: char) -> String {
        match c {
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
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("invalid hex '{}'", c),
        }
        .to_string()
    }

    /// Converts a string containing hexadecimal repreesntation its corresponding
    /// binary representation.
    ///
    /// * `s` - The hexadecimal string.
    fn hex_str_to_bin(s: &str) -> String {
        let mut bin = String::new();
        for c in s.chars().into_iter() {
            bin.push_str(&Self::hex_char_to_bin(c));
        }
        bin
    }

    /// Converts a string containing binary repreesntation its corresponding
    /// decimal representation.
    ///
    /// * `s` - The hexadecimal string.
    fn bin_str_to_decimal(s: &str) -> u8 {
        u8::from_str_radix(s, 2).unwrap()
    }

    /// Returns a list of versions in the decoded packet hierarchy.
    fn get_versions(&self) -> Vec<u8> {
        match self {
            Self::Literal(version, _) => vec![*version],
            Self::Operator(version, _, packets) => {
                let mut versions: Vec<u8> = packets.iter().flat_map(|p| p.get_versions()).collect();
                versions.push(*version);
                versions
            }
        }
    }

    /// Recursively evaluates the expression in the given packet.
    fn evaluate(&self) -> Result<u64, String> {
        match self {
            Self::Literal(_, value) => Ok(*value),
            Self::Operator(_, Op::Invalid(op), _) => {
                Err(format!("cannot evaluate invalid operation '{}'", op))
            }
            Self::Operator(_, Op::Sum, packets) => {
                let mut result = 0_u64;
                for p in packets {
                    let v = p.evaluate()?;
                    result += v;
                }
                Ok(result)
            }
            Self::Operator(_, Op::Product, packets) => {
                let mut result = 1_u64;
                for p in packets {
                    let v = p.evaluate()?;
                    result *= v;
                }
                Ok(result)
            }
            Self::Operator(_, Op::Minimum, packets) => {
                let mut result = packets[0].evaluate()?;
                for p in packets.iter().skip(1) {
                    let v = p.evaluate()?;
                    result = result.min(v);
                }
                Ok(result)
            }
            Self::Operator(_, Op::Maximum, packets) => {
                let mut result = packets[0].evaluate()?;
                for p in packets.iter().skip(1) {
                    let v = p.evaluate()?;
                    result = result.max(v);
                }
                Ok(result)
            }
            Self::Operator(_, op, packets) => {
                let a = packets[0].evaluate();
                let b = packets[1].evaluate();
                match op {
                    Op::GreaterThan => Ok((a > b) as u64),
                    Op::LessThan => Ok((a < b) as u64),
                    Op::Equal => Ok((a == b) as u64),
                    _ => unreachable!(),
                }
            }
        }
    }

    /// Pretty prints the packets.
    ///
    /// * `f`     - Formatter
    /// * `level` - Used for indentation.
    fn print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            Self::Literal(version, value) => {
                write!(f, "{{Ver: {}, Type: Literal, Value: {}}}", version, value)
            }
            Self::Operator(version, packet_type, packets) => {
                write!(
                    f,
                    "{{Ver: {}, Type: Operator({}), Packets: [\n",
                    version, packet_type,
                )?;
                for i in 0..packets.len() {
                    write!(f, "{}", repeat("  ").take(level + 1).collect::<String>())?;
                    packets[i].print(f, level + 1)?;
                    write!(f, ",\n")?;
                }
                write!(f, "{}", repeat("  ").take(level).collect::<String>())?;
                write!(f, "]}}")?;
                Ok(())
            }
        }
    }
}

impl fmt::Display for Packet {
    /// Formats the value using the given formatter.
    ///
    /// * `f`     - Formatter
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print(f, 0)
    }
}

fn read(input_file: &str) -> Vec<String> {
    let content = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    content.split("\n").map(|s| s.to_string()).collect()
}

pub fn part1(input_file: &str) {
    println!("day 16: part 1");
    let tx = read(input_file);
    for s in tx {
        let (p, _) = Packet::parse(&s);
        let sum: usize = p.get_versions().iter().map(|&v| v as usize).sum();
        if s.len() < 20 {
            println!("{}", s);
        } else {
            println!("{}...", &s[0..20]);
        }
        println!("{}", p);
        println!("sum versions = {}", sum);
        println!();
    }
}

pub fn part2(input_file: &str) {
    println!("day 16: part 2");
    let tx = read(input_file);
    for s in tx {
        let (p, _) = Packet::parse(&s);
        match p.evaluate() {
            Ok(r) => {
                if s.len() < 20 {
                    println!("{} = {}", s, r);
                } else {
                    println!("{}... = {}", &s[0..20], r);
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
