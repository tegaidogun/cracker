pub fn echo(args: &[&str]) -> Result<(), String> {
    let mut no_newline = false;
    let mut interpret_escapes = false;

    let mut output = String::new();

    for arg in args {
        match *arg {
            "-n" => no_newline = true,
            "-E" => interpret_escapes = false,
            "-e" => interpret_escapes = true,
            _ => {
                if interpret_escapes {
                    output.push_str(&interpret_escape_sequences(arg));
                } else {
                    output.push_str(arg);
                }
                output.push(' ');
            }
        }
    }

    if !no_newline {
        output.push('\n');
    } else {
        // Remove the last trailing space if no_newline is set
        output.pop();
    }

    print!("{}", output);

    Ok(())
}

fn interpret_escape_sequences(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.peek() {
                Some('a') => result.push('\x07'), // Alert (bell)
                Some('b') => result.push('\x08'), // Backspace
                Some('c') => return result,       // Suppress trailing newline
                Some('e') | Some('E') => result.push('\x1B'), // Escape
                Some('f') => result.push('\x0C'), // Form feed
                Some('n') => result.push('\n'),   // New line
                Some('r') => result.push('\r'),   // Carriage return
                Some('t') => result.push('\t'),   // Horizontal tab
                Some('v') => result.push('\x0B'), // Vertical tab
                Some('\\') => result.push('\\'),  // Backslash
                Some('0') => {
                    let mut octal = String::new();
                    chars.next();
                    while let Some(digit) = chars.peek() {
                        if digit.is_digit(8) && octal.len() < 3 {
                            octal.push(*digit);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(octal_value) = u8::from_str_radix(&octal, 8) {
                        result.push(octal_value as char);
                    } else {
                        result.push_str("\\0");
                        result.push_str(&octal);
                    }
                }
                Some('x') => {
                    let mut hex = String::new();
                    chars.next();
                    while let Some(digit) = chars.peek() {
                        if digit.is_digit(16) && hex.len() < 2 {
                            hex.push(*digit);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(hex_value) = u8::from_str_radix(&hex, 16) {
                        result.push(hex_value as char);
                    } else {
                        result.push_str("\\x");
                        result.push_str(&hex);
                    }
                }
                Some('u') => {
                    let mut hex = String::new();
                    chars.next();
                    while let Some(digit) = chars.peek() {
                        if digit.is_digit(16) && hex.len() < 4 {
                            hex.push(*digit);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(hex_value) = u16::from_str_radix(&hex, 16) {
                        result.push(char::from_u32(hex_value as u32).unwrap_or('\u{FFFD}'));
                    } else {
                        result.push_str("\\u");
                        result.push_str(&hex);
                    }
                }
                Some('U') => {
                    let mut hex = String::new();
                    chars.next();
                    while let Some(digit) = chars.peek() {
                        if digit.is_digit(16) && hex.len() < 8 {
                            hex.push(*digit);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(hex_value) = u32::from_str_radix(&hex, 16) {
                        result.push(char::from_u32(hex_value).unwrap_or('\u{FFFD}'));
                    } else {
                        result.push_str("\\U");
                        result.push_str(&hex);
                    }
                }
                _ => result.push(ch),
            }
            chars.next(); // Move past the escape character
        } else {
            result.push(ch);
        }
    }

    result
}
