//! <https://adventofcode.com/2021/day/16>
//!
//! Representing bits as chars is inefficient (char is four bytes large), but convenient.

const INPUT: &str = "C20D718021600ACDC372CD8DE7A057252A49C940239D68978F7970194EA7CCB310088760088803304A0AC1B100721EC298D3307440041CD8B8005D12DFD27CBEEF27D94A4E9B033006A45FE71D665ACC0259C689B1F99679F717003225900465800804E39CE38CE161007E52F1AEF5EE6EC33600BCC29CFFA3D8291006A92CA7E00B4A8F497E16A675EFB6B0058F2D0BD7AE1371DA34E730F66009443C00A566BFDBE643135FEDF321D000C6269EA66545899739ADEAF0EB6C3A200B6F40179DE31CB7B277392FA1C0A95F6E3983A100993801B800021B0722243D00042E0DC7383D332443004E463295176801F29EDDAA853DBB5508802859F2E9D2A9308924F9F31700AA4F39F720C733A669EC7356AC7D8E85C95E123799D4C44C0109C0AF00427E3CC678873F1E633C4020085E60D340109E3196023006040188C910A3A80021B1763FC620004321B4138E52D75A20096E4718D3E50016B19E0BA802325E858762D1802B28AD401A9880310E61041400043E2AC7E8A4800434DB24A384A4019401C92C154B43595B830002BC497ED9CC27CE686A6A43925B8A9CFFE3A9616E5793447004A4BBB749841500B26C5E6E306899C5B4C70924B77EF254B48688041CD004A726ED3FAECBDB2295AEBD984E08E0065C101812E006380126005A80124048CB010D4C03DC900E16A007200B98E00580091EE004B006902004B00410000AF00015933223100688010985116A311803D05E3CC4B300660BC7283C00081CF26491049F3D690E9802739661E00D400010A8B91F2118803310A2F43396699D533005E37E8023311A4BB9961524A4E2C027EC8C6F5952C2528B333FA4AD386C0A56F39C7DB77200C92801019E799E7B96EC6F8B7558C014977BD00480010D89D106240803518E31C4230052C01786F272FF354C8D4D437DF52BC2C300567066550A2A900427E0084C254739FB8E080111E0";

enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

enum Packet {
    Literal(
        /// Version.
        u8,
        /// Value.
        u128,
    ),
    Operator(
        /// Version.
        u8,
        Operation,
        /// Sub-packets.
        Vec<Packet>,
    ),
}

impl Operation {
    fn from_type_id(type_id: u8) -> Operation {
        match type_id {
            0 => Operation::Sum,
            1 => Operation::Product,
            2 => Operation::Minimum,
            3 => Operation::Maximum,
            5 => Operation::GreaterThan,
            6 => Operation::LessThan,
            7 => Operation::EqualTo,
            _ => panic!("Invalid operation"),
        }
    }
}

impl Packet {
    fn version_sum(&self) -> u32 {
        match self {
            Packet::Literal(version, _) => *version as u32,
            Packet::Operator(version, _, subpackets) => {
                *version as u32 + subpackets.iter().map(|p| p.version_sum()).sum::<u32>()
            }
        }
    }

    fn evaluate(&self) -> u128 {
        match self {
            Packet::Literal(_, value) => *value,
            Packet::Operator(_, operation, subpackets) => {
                let subpacket_values: Vec<u128> = subpackets.iter().map(|p| p.evaluate()).collect();
                match operation {
                    Operation::Sum => subpacket_values.iter().sum(),
                    Operation::Product => subpacket_values.iter().product(),
                    Operation::Minimum => *subpacket_values.iter().min().unwrap(),
                    Operation::Maximum => *subpacket_values.iter().max().unwrap(),
                    Operation::GreaterThan => (subpacket_values[0] > subpacket_values[1]) as u128,
                    Operation::LessThan => (subpacket_values[0] < subpacket_values[1]) as u128,
                    Operation::EqualTo => (subpacket_values[0] == subpacket_values[1]) as u128,
                }
            }
        }
    }
}

/// Returns a tuple where:
/// * First element is the packet parsed from the given bit vector starting at the given index.
/// * Second element is the index to first bit after the parsed packet.
fn parse_packet(bits: &Vec<char>, mut i: usize) -> (Packet, usize) {
    let version = u8::from_str_radix(&bits[i..i + 3].iter().collect::<String>(), 2).unwrap();
    let type_id = u8::from_str_radix(&bits[i + 3..i + 6].iter().collect::<String>(), 2).unwrap();
    i += 6;

    match type_id {
        4 => {
            let mut value_bit_string = String::new();
            loop {
                let last_group_indicator = bits[i];
                let group_bits = &bits[i + 1..i + 5];
                i += 5;

                value_bit_string.push_str(&group_bits.iter().collect::<String>());

                if last_group_indicator == '0' {
                    break;
                }
            }
            let value = u128::from_str_radix(&value_bit_string, 2).unwrap();

            let packet = Packet::Literal(version, value);
            (packet, i)
        }
        _ => {
            let length_type_id = bits[i];
            i += 1;

            let mut subpackets = Vec::new();
            match length_type_id {
                '0' => {
                    let subpackets_total_len =
                        usize::from_str_radix(&bits[i..i + 15].iter().collect::<String>(), 2)
                            .unwrap();
                    i += 15;

                    let subpackets_end_i = i + subpackets_total_len;
                    loop {
                        let (subpacket, new_i) = parse_packet(bits, i);
                        subpackets.push(subpacket);
                        i = new_i;

                        if i == subpackets_end_i {
                            break;
                        }
                    }
                }
                '1' => {
                    let num_subpackets =
                        u16::from_str_radix(&bits[i..i + 11].iter().collect::<String>(), 2)
                            .unwrap();
                    i += 11;

                    for _ in 0..num_subpackets {
                        let (subpacket, new_i) = parse_packet(bits, i);
                        subpackets.push(subpacket);
                        i = new_i;
                    }
                }
                _ => panic!("Invalid bit"),
            }

            let packet = Packet::Operator(version, Operation::from_type_id(type_id), subpackets);
            (packet, i)
        }
    }
}

fn parse_transmission(hex_string: &str) -> Packet {
    let bit_string = hex_string
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|d| format!("{:04b}", d))
        .collect::<Vec<String>>()
        .join("");
    parse_packet(&bit_string.chars().collect(), 0).0
}

pub fn solution_1() -> String {
    parse_transmission(INPUT).version_sum().to_string()
}

pub fn solution_2() -> String {
    parse_transmission(INPUT).evaluate().to_string()
}
