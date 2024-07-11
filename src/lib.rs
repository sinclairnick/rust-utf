/// Given an input array of bytes, converts the bytes into UTF-8
/// format by manually iterating over the bits
pub fn parse_bits(bytes: &[u8]) -> Result<Vec<char>, &str> {
    let mut chars: Vec<char> = Vec::new();

    // cp = code point
    let mut cp_bits: u32 = 0b_0; // Buf bits should never overflow w=21
    let mut cp_remaining_bytes = 0;

    for b in bytes {
        println!(">{b:08b}");
        let mut prefix_ones_count = 0_i32;
        let mut has_parsed_prefix_bits = false;

        for i in (0..8).rev() {
            let val = (*b >> i) & 1;
            // Tally up the prefix bits, skipping the "real" processing below
            if !has_parsed_prefix_bits {
                if val == 1 {
                    prefix_ones_count += 1;
                    continue;
                }

                has_parsed_prefix_bits = true;
                continue;
            }

            // Initialise remaining_bytes value
            if cp_remaining_bytes == 0 && prefix_ones_count > 1 {
                // Set the remaining bytes count
                cp_remaining_bytes = prefix_ones_count;
            }

            let is_continuation_byte = prefix_ones_count == 1;

            if is_continuation_byte {
                if cp_remaining_bytes == 0 {
                    return Err("New code point byte expected. Found continuation byte.");
                }
            } else if prefix_ones_count != cp_remaining_bytes && cp_remaining_bytes > 0 {
                // If this is a non-continuation byte and there
                // are remaining bytes expected, also error
                return Err("Continuation byte expected. Found new code point byte.");
            }

            // Append bit into RHS of current bits
            cp_bits = (cp_bits << 1) | val as u32;
            println!("Buffer: {cp_bits:08b}")
        }

        println!("Remaining: {cp_remaining_bytes}");

        // Convert from raw bits to a UTF-8 char
        let bits_as_char =
            char::from_u32(cp_bits).expect("Could not parse code point bits to UTF char.");
        println!("Current char: {bits_as_char}");

        // Decrement the remaining bytes count
        cp_remaining_bytes -= 1;

        let is_last_cp_byte = cp_remaining_bytes < 1;

        if is_last_cp_byte {
            chars.push(bits_as_char);

            // Reset bits and remaining count
            cp_bits = 0b_0;
            cp_remaining_bytes = 0;
        }

        println!("");
    }

    return Ok(chars);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_char() {
        let bytes = "a".as_bytes();
        let result = parse_bits(bytes);
        let exp = Vec::from(['a']);

        assert_eq!(result.unwrap(), exp);
    }

    #[test]
    fn multi_char() {
        let bytes = "ab".as_bytes();
        let result = parse_bits(bytes);
        let exp = Vec::from(['a', 'b']);

        assert_eq!(result.unwrap(), exp);
    }

    #[test]
    fn non_ascii() {
        let bytes = "¥".as_bytes();
        let result = parse_bits(bytes);
        let exp = Vec::from(['¥']);

        assert_eq!(result.unwrap(), exp);
    }

    #[test]
    fn non_ascii_multi() {
        let bytes = "¥∫".as_bytes();
        let result = parse_bits(bytes);
        let exp = Vec::from(['¥', '∫']);

        assert_eq!(result.unwrap(), exp);
    }
}