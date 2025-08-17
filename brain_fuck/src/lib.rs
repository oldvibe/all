pub fn brain_fuck(code: &str) {
    let chars: Vec<char> = code.chars().collect();
    let mut memory = vec![0u8; 2048];
    let mut index: usize = 0;
    let mut i: usize = 0;

    while i < chars.len() {
        match chars[i] {
            '>' => index += 1,
            '<' => index -= 1,
            '+' => memory[index] += 1,
            '-' => memory[index] -= 1,
            '.' => print!("{}", memory[index] as char),
            '[' => {
                if memory[index] == 0 {
                    let mut dep = 1;
                    while dep > 0 {
                        i += 1;
                        if chars[i] == '[' {
                            dep += 1;
                        } else if chars[i] == ']' {
                            dep -= 1;
                        }
                    }
                }
            }
            ']' => {
                if memory[index] != 0 {
                    let mut dep = 1;
                    while dep > 0 {
                        i -= 1;
                        if chars[i] == ']' {
                            dep += 1;
                        } else if chars[i] == '[' {
                            dep -= 1;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
}
