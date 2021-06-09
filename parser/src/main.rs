#[macro_use]
extern crate nom;

use nom::{
    character::complete::{space1, space0, digit1, hex_digit1, oct_digit1},
    bytes::complete::tag_no_case,
    IResult,
};

use arch::registers;

use core::{fmt::Debug};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParameterType {
    RegisterU8,
    RegisterU16,
    Literal
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterU8 {
    reg_type: u8,
    reg_name: String,
}

impl RegisterU8 {
    pub fn new(reg_type: u8, reg_name: String) -> Self {
        Self {
            reg_type,
            reg_name
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Registeru16 {
    reg_type: u8,
    reg_name: String,
}

impl Registeru16 {
    pub fn new(reg_type: u8, reg_name: String) -> Self {
        Self {
            reg_type,
            reg_name
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Literal {
    value: u16
}

#[derive(Debug)]
pub enum Parameter {
    RegU16{name: String, reg_id: u8},
    RegU8{name: String, reg_id: u8},
    Lit(u16),
}

#[derive(Debug)]
struct Instruction {
    ins_name: String,
    param1: Parameter,
    param2: Parameter,
}

impl Instruction {
    pub fn new(ins_name:&str, param1: Parameter, param2: Parameter) -> Self {
        Self {
            ins_name: ins_name.to_owned(),
            param1,
            param2,
        }
    }
}

#[derive(Default, Debug)]
struct Program {
    instructions: Vec<Instruction>
}

named!(reg<&str, &str>, alt!(tag_no_case!("ah") | tag_no_case!("al") | tag_no_case!("ax") |
    tag_no_case!("bh") | tag_no_case!("bl") | tag_no_case!("bx") |
    tag_no_case!("ch") | tag_no_case!("cl") | tag_no_case!("cx") |
    tag_no_case!("dh") | tag_no_case!("dl") | tag_no_case!("dx") |
    tag_no_case!("ex") | tag_no_case!("fx") | tag_no_case!("gx") | tag_no_case!("hx") | tag_no_case!("acc")
));

named!(ins<&str, &str>, alt!(
    tag_no_case!("mov") | tag_no_case!("add") |
    tag_no_case!("sub") | tag_no_case!("mul")
));

named!(reg_8<&str, &str>, alt!(
    tag_no_case!("ah") | tag_no_case!("al") |
    tag_no_case!("bh") | tag_no_case!("bl") |
    tag_no_case!("ch") | tag_no_case!("cl") |
    tag_no_case!("dh") | tag_no_case!("dl")
));

named!(reg_16<&str, &str>, alt!(
    tag_no_case!("ax") | tag_no_case!("bx") |
    tag_no_case!("cx") | tag_no_case!("dx") |
    tag_no_case!("ex") | tag_no_case!("fx") |
    tag_no_case!("gx") | tag_no_case!("hx") |
    tag_no_case!("acc")
));

named!(lit<&str, u16>, alt!(
    do_parse!(_prefix: tag_no_case!("0o") >> value: oct_digit1 >> (u16::from_str_radix(value, 8).unwrap())) |
    do_parse!(_prefix: tag_no_case!("0x") >> value: hex_digit1 >> (u16::from_str_radix(value, 16).unwrap())) |
    do_parse!(value: digit1 >> (value.parse::<u16>().unwrap()))
));

named!(get_reg<&str, Parameter>, alt!(
    do_parse!(name: reg_8 >> (Parameter::RegU8{name: name.to_owned(), reg_id: registers::REGISTER_NAMES.iter().position(|&n| n == name).unwrap() as u8})) |
    do_parse!(name: reg_16 >> (Parameter::RegU16{name: name.to_owned(), reg_id: registers::REGISTER_NAMES.iter().position(|&n| n == name).unwrap() as u8})) |
    do_parse!(lit: lit >> (Parameter::Lit(lit)))
));

named!(get_ins<&str, Instruction>, do_parse!(
    name: terminated!(ins, space1) >>
    r1: terminated!(get_reg, space1) >>
    r2: terminated!(get_reg, space0) >>
    (Instruction::new(name, r1, r2))
));

pub fn upper_or_lower_str<'a>(to_match: &'a str, input: &'a str) -> IResult<&'a str, &'a str>  {
    tag_no_case(to_match)(input)
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_upper_string_ok() {
    let input_text = "mov x y";
    let input_text2 = "MOV X Y";

    let output = upper_or_lower_str("mov", input_text);
    let output2 = upper_or_lower_str("mov", input_text2);

    dbg!(&output);
    dbg!(&output2);
    let expected = Ok((" x y", "mov"));
    let expected2 = Ok((" X Y", "MOV"));
    assert_eq!(output, expected);
    assert_eq!(output2, expected2);
}

#[test]
fn test_mov_reg_reg() {
    let input = "mov 0x4f bh";

    let (r, ins) = get_ins(input).unwrap();
    assert_eq!(r, "");
    dbg!(r);
    dbg!(ins);
}
