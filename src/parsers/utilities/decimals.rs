pub fn from_decimal(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 10)
}

pub fn is_decimal_digit(character: char) -> bool {
  character.is_digit(10)
}
