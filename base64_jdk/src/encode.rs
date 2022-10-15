// This code is taken from JDK17 just for the benchmark purposes.

use crate::{Config, PAD_BYTE};
use crate::tables::{TO_BASE64_STANDARD, TO_BASE64_URL_SAFE};

pub fn encode<T: AsRef<[u8]>>(input: T, config: Config) -> Vec<u8> {
    let src = input.as_ref();
    let mut buffer = match calculate_output_length(src, config) {
        Some(len) => vec![0; len],
        None => panic!("Encoded size is too large"),
    };

    let table = if config.url_safe { TO_BASE64_URL_SAFE } else { TO_BASE64_STANDARD };

    let mut sp = 0;
    let sl = src.len();
    let slen = sl / 3 * 3;

    let mut dp = 0;
    let mut write = |b: u8| {
        buffer[dp] = b;
        dp += 1;
    };

    while sp < slen {
        let bits = (src[sp + 0] as usize) << 16 |
            (src[sp + 1] as usize) << 8 |
            src[sp + 2] as usize;
        sp += 3;
        write(table[(bits >> 18) & 0x3f]);
        write(table[(bits >> 12) & 0x3f]);
        write(table[(bits >> 6) & 0x3f]);
        write(table[bits & 0x3f]);
    }

    if sp < sl { // 1 or 2 leftover bytes
        let b0 = src[sp];
        sp += 1;
        write(table[b0 as usize >> 2]);
        if sp == sl {
            write(table[((b0 as usize) << 4) & 0x3f]);
            if config.pad {
                write(PAD_BYTE);
                write(PAD_BYTE);
            }
        } else {
            let b1 = src[sp];
            //sp += 1;
            write(table[((b0 as usize) << 4) & 0x3f | ((b1 as usize) >> 4)]);
            write(table[((b1 as usize) << 2) & 0x3f]);
            if config.pad {
                write(PAD_BYTE);
            }
        }
    }

    buffer
}

fn calculate_output_length(input: &[u8], config: Config) -> Option<usize> {
    if config.pad {
        // ((len + 2) / 3) * 4
        input.len().checked_add(2).and_then(|v| v.checked_div(3)).and_then(|v| v.checked_mul(4))
    } else {
        let n = input.len() % 3;
        // (len / 3) * 4 + (n == 0 ? 0 : n + 1)
        input.len().checked_div(3)
            .and_then(|v| v.checked_mul(4))
            .and_then(|v| if n == 0 { Some(v) } else { v.checked_add(n + 1) })
    }
}

#[cfg(test)]
mod tests {
    use crate::STANDARD;

    use super::*;

    #[test]
    fn should_calculate_size_successfully() {
        assert_eq!(calculate_output_length("".as_bytes(), STANDARD), Some(0));
        assert_eq!(calculate_output_length("a".as_bytes(), STANDARD), Some(4));
        assert_eq!(calculate_output_length("abcd".as_bytes(), STANDARD), Some(8));
        assert_eq!(calculate_output_length("nmmфывыаывпврапопррло".as_bytes(), STANDARD), Some(52));
    }

    #[test]
    fn should_encode_successfully_empty_string() {
        assert_encode("", "", STANDARD);
    }

    #[test]
    fn should_decode_successfully_single_character() {
        assert_encode("YQ==", "a", STANDARD);
    }

    #[test]
    fn should_decode_successfully_four_characters() {
        assert_encode("YWJjZA==", "abcd", STANDARD);
    }

    #[test]
    fn should_decode_successfully_utf8() {
        assert_encode("bm1t0YTRi9Cy0YvQsNGL0LLQv9Cy0YDQsNC/0L7Qv9GA0YDQu9C+", "nmmфывыаывпврапопррло", STANDARD);
    }

    fn assert_encode(expected: &str, input: &str, config: Config) {
        let encoded = encode(input, config);
        assert_eq!(expected.to_string(), String::from_utf8(encoded).unwrap());
    }
}
