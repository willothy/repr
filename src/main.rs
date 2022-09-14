use std::env;

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Format {
    Binary = 0,
    Decimal = 1,
    Hexadecimal = 2,
    Octal = 3,
}

impl Format {
    pub fn name(&self) -> &'static str {
        match self {
            Format::Binary => "bin",
            Format::Decimal => "dec",
            Format::Hexadecimal => "hex",
            Format::Octal => "oct",
        }
    }

    pub fn prefix(&self) -> String {
        match self {
            Format::Binary => "0b",
            Format::Decimal => "",
            Format::Hexadecimal => "0x",
            Format::Octal => "0o",
        }
        .to_owned()
    }

    pub fn encode(&self, num: u64) -> String {
        match self {
            Format::Binary => format!("{:b}", num),
            Format::Decimal => format!("{}", num),
            Format::Hexadecimal => format!("{:x}", num),
            Format::Octal => format!("{:o}", num),
        }
    }

    pub fn show(&self, num: u64) {
        println!("{:3} {}{}", self.name(), self.prefix(), self.encode(num));
    }
}

impl From<u8> for Format {
    fn from(value: u8) -> Self {
        match value {
            0 => Format::Binary,
            1 => Format::Decimal,
            2 => Format::Hexadecimal,
            3 => Format::Octal,
            _ => panic!("Invalid format"),
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let num = &args[1];
    let format: Format;

    let num_int = {
        if num.starts_with("0x") {
            format = Format::Hexadecimal;
            u64::from_str_radix(&num[2..], 16)
        } else if num.starts_with("0b") {
            format = Format::Binary;
            u64::from_str_radix(&num[2..], 2)
        } else if num.starts_with("0o") {
            format = Format::Octal;
            u64::from_str_radix(&num[2..], 8)
        } else {
            format = Format::Decimal;
            num.parse::<u64>()
        }
    };

    if let Ok(num) = num_int {
        println!(
            "{} {}{:10}{}",
            format.name(),
            format.prefix(),
            format.encode(num),
            "(input)"
        );
        for i in 0..=3 {
            if i == format.clone() as u8 {
                continue;
            }
            let format: Format = Format::from(i);
            format.show(num);
        }
    } else {
        println!("Invalid number: {}", num);
    }
}
