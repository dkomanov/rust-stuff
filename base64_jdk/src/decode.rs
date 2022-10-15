/// This code is taken from JDK17 just for the benchmark purposes.

use crate::{Config, PAD_BYTE};
use crate::tables::{DecodeTable, FROM_BASE64_STANDARD, FROM_BASE64_URL_SAFE};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    InputShouldBeAtLeast2BytesLong,
    InvalidCharacter(u8, usize),
    InvalidFourByteEndingUnit,
    LastUnitDoesntHaveEnoughValidBits,
    BufferIsTooSmall(usize, usize),
}

pub fn decode<T: AsRef<[u8]>>(input: T, config: Config) -> Result<Vec<u8>, DecodeError> {
    let input_bytes = input.as_ref();
    let output_size = calculate_output_length(input_bytes)?;
    let mut output = vec![0; output_size];
    decode_slice(input_bytes, config, &mut output)?;
    Ok(output)
}

fn decode_block(input: &[u8], current_sp: usize, sl: usize, dp: usize, output: &mut [u8], table: &DecodeTable) -> (usize, usize) {
    let mut sp = current_sp;
    let mut new_dp = dp;
    let sl0 = sp + ((sl - sp) & !0b11usize);
    while sp < sl0 {
        let b1 = table[input[sp + 0] as usize] as u32;
        let b2 = table[input[sp + 1] as usize] as u32;
        let b3 = table[input[sp + 2] as usize] as u32;
        let b4 = table[input[sp + 3] as usize] as u32;
        if (b1 | b2 | b3 | b4) > 127 {
            return (sp, new_dp);
        }

        let bits0 = b1 << 18 | b2 << 12 | b3 << 6 | b4;
        output[new_dp + 0] = (bits0 >> 16) as u8;
        output[new_dp + 1] = (bits0 >> 8) as u8;
        output[new_dp + 2] = (bits0 >> 0) as u8;

        new_dp += 3;
        sp += 4;
    }
    (sp, new_dp)
}

pub fn decode_slice<T: AsRef<[u8]>>(input: T, config: Config, output: &mut [u8]) -> Result<usize, DecodeError> {
    let input_bytes = input.as_ref();
    // TODO Rename to table
    let base64 = if config.url_safe { FROM_BASE64_URL_SAFE } else { FROM_BASE64_STANDARD };

    let mut dp = 0;
    let mut bits = 0u32;
    let mut shiftto = 18i32; // pos of first byte of 4-byte atom

    let mut sp = 0;
    let sl = input_bytes.len();

    while sp < sl {
        if shiftto == 18 && sp < sl - 4 { // fast path
            (sp, dp) = decode_block(input_bytes, sp, sl, dp, output, base64);

            if sp >= sl {
                break;
            }
        }

        let b = input_bytes[sp];
        sp += 1;

        let decoded = base64[b as usize];
        match decoded {
            255 => return Err(DecodeError::InvalidCharacter(b, sp - 1)),
            254 => {
                if shiftto == 6 && (sp == sl || input_bytes[sp] != PAD_BYTE) || shiftto == 18 {
                    return Err(DecodeError::InvalidFourByteEndingUnit);
                }

                if sp != sl {
                    sp += 1;
                }

                break;
            }
            _ => (),
        }

        bits |= (decoded as u32) << shiftto;
        shiftto -= 6;
        if shiftto < 0 {
            output[dp + 0] = (bits >> 16) as u8;
            output[dp + 1] = (bits >> 8) as u8;
            output[dp + 2] = (bits >> 0) as u8;
            dp += 3;
            shiftto = 18;
            bits = 0;
        }
    }

    if shiftto == 6 {
        output[dp] = (bits >> 16) as u8;
        dp += 1;
    } else if shiftto == 0 {
        output[dp + 0] = (bits >> 16) as u8;
        output[dp + 1] = (bits >> 8) as u8;
        dp += 2;
    } else if shiftto == 12 {
        return Err(DecodeError::LastUnitDoesntHaveEnoughValidBits);
    }

    if sp < sl {
        return Err(DecodeError::InvalidCharacter(input_bytes[sp], sp));
    }

    Ok(dp)
}

pub fn calculate_output_length(input: &[u8]) -> Result<usize, DecodeError> {
    let len = input.len();
    if len == 0 {
        return Ok(0);
    } else if len < 2 {
        return Err(DecodeError::InputShouldBeAtLeast2BytesLong);
    }

    let paddings: u64 = if input[len - 1] == PAD_BYTE {
        if input[len - 2] == PAD_BYTE { 2 } else { 1 }
    } else if len & 0x3 != 0 {
        4 - (len as u64 & 0x3)
    } else {
        0
    };

    // If len is near to Integer.MAX_VALUE, (len + 3)
    // can possibly overflow, perform this operation as
    // long and cast it back to integer when the value comes under
    // integer limit. The final value will always be in integer
    // limits
    Ok((3 * ((len as u64 + 3) / 4) - paddings) as usize)
}

#[cfg(test)]
mod tests {
    use crate::STANDARD;

    use super::*;

    #[test]
    fn should_calculate_size_successfully() {
        assert_eq!(calculate_output_length("".as_bytes()), Ok(0));
        assert_eq!(calculate_output_length("YQ==".as_bytes()), Ok(1));
        assert_eq!(calculate_output_length("YWJjZA==".as_bytes()), Ok(4));
        assert_eq!(calculate_output_length("bm1t0YTRi9Cy0YvQsNGL0LLQv9Cy0YDQsNC/0L7Qv9GA0YDQu9C+".as_bytes()), Ok(35));
    }

    #[test]
    fn should_decode_successfully_empty_string() {
        assert_eq!(decode("", STANDARD), Ok(Vec::from("")));
    }

    #[test]
    fn should_decode_successfully_single_character() {
        assert_eq!(decode("YQ==", STANDARD), Ok(Vec::from("a")));
    }

    #[test]
    fn should_decode_successfully_four_characters() {
        assert_eq!(decode("YWJjZA==", STANDARD), Ok(Vec::from("abcd")));
    }

    #[test]
    fn should_decode_successfully_utf8() {
        assert_eq!(decode("bm1t0YTRi9Cy0YvQsNGL0LLQv9Cy0YDQsNC/0L7Qv9GA0YDQu9C+", STANDARD), Ok(Vec::from("nmmфывыаывпврапопррло")));
    }
}
