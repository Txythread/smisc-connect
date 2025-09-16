use crate::util::stc::*;

/// Create STC values from a string and ignore all characters that can't be converted
pub fn ascii_string_to_stc(text: String) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for c in text.chars() {
        if let Some(stc_value) = STCValue::from_ascii(c){
            output.push(stc_value.get_value());
        }
    }

    output
}

pub fn stc_to_string(stc: Vec<u8>) -> String {
    let mut output = String::new();

    for byte in stc {
        if let Some(stc_value) = STCValue::from_value(byte){
            if let Some(character) = stc_value.to_ascii(){
                output.push(character);
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::util::stc_conversion::{ascii_string_to_stc, stc_to_string};

    #[test]
    fn test_ascii_string_to_stc() {
        let test_string = String::from("abc");

        let expected_stc = vec![
            0b1010_0100u8,
            0b1010_0101u8,
            0b1010_0110u8,
        ];

        assert_eq!(ascii_string_to_stc(test_string), expected_stc);
    }

    #[test]
    fn test_stc_to_string() {
        let test_stc_values = vec![
            0b1010_0100u8,
            0b1010_0101u8,
            0b1010_0110u8,
        ];

        let expected_stc = String::from("abc");

        assert_eq!(stc_to_string(test_stc_values), expected_stc);
    }
}