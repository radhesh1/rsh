mod and;
mod bits_;
mod into;
mod not;
mod or;
mod rotate_left;
mod rotate_right;
mod shift_left;
mod shift_right;
mod xor;

pub use and::BitsAnd;
pub use bits_::Bits;
pub use into::BitsInto;
pub use not::BitsNot;
pub use or::BitsOr;
pub use rotate_left::BitsRol;
pub use rotate_right::BitsRor;
pub use shift_left::BitsShl;
pub use shift_right::BitsShr;
pub use xor::BitsXor;

use rsh_protocol::Spanned;

#[derive(Clone, Copy)]
enum numberBytes {
    One,
    Two,
    Four,
    Eight,
    Auto,
    Invalid,
}

#[derive(Clone, Copy)]
enum InputnumType {
    One,
    Two,
    Four,
    Eight,
    SignedOne,
    SignedTwo,
    SignedFour,
    SignedEight,
}

fn get_number_bytes(number_bytes: Option<&Spanned<String>>) -> numberBytes {
    match number_bytes.as_ref() {
        None => numberBytes::Eight,
        Some(size) => match size.item.as_str() {
            "1" => numberBytes::One,
            "2" => numberBytes::Two,
            "4" => numberBytes::Four,
            "8" => numberBytes::Eight,
            "auto" => numberBytes::Auto,
            _ => numberBytes::Invalid,
        },
    }
}

fn get_input_num_type(val: i64, signed: bool, number_size: numberBytes) -> InputnumType {
    if signed || val < 0 {
        match number_size {
            numberBytes::One => InputnumType::SignedOne,
            numberBytes::Two => InputnumType::SignedTwo,
            numberBytes::Four => InputnumType::SignedFour,
            numberBytes::Eight => InputnumType::SignedEight,
            numberBytes::Auto => {
                if val <= 0x7F && val >= -(2i64.pow(7)) {
                    InputnumType::SignedOne
                } else if val <= 0x7FFF && val >= -(2i64.pow(15)) {
                    InputnumType::SignedTwo
                } else if val <= 0x7FFFFFFF && val >= -(2i64.pow(31)) {
                    InputnumType::SignedFour
                } else {
                    InputnumType::SignedEight
                }
            }
            numberBytes::Invalid => InputnumType::SignedFour,
        }
    } else {
        match number_size {
            numberBytes::One => InputnumType::One,
            numberBytes::Two => InputnumType::Two,
            numberBytes::Four => InputnumType::Four,
            numberBytes::Eight => InputnumType::Eight,
            numberBytes::Auto => {
                if val <= 0xFF {
                    InputnumType::One
                } else if val <= 0xFFFF {
                    InputnumType::Two
                } else if val <= 0xFFFFFFFF {
                    InputnumType::Four
                } else {
                    InputnumType::Eight
                }
            }
            numberBytes::Invalid => InputnumType::Four,
        }
    }
}
