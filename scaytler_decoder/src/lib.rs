pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
      if s.is_empty() || letters_per_turn == 0 {
          return None;
      }
  
      let decoded = decode(&s, letters_per_turn as usize);
      if decoded.is_empty() {
          return None;
      }
  
      Some(decoded)
  }
  
  fn decode(s: &str, columns: usize) -> String {
      let mut start: usize = 0;
      let mut decoded = "".to_string();
      while decoded.len() < s.len() {
            let mut next = start + columns;
            decoded.push(s.chars().nth(start).unwrap());
            while next < s.len() {
                  decoded.push(s.chars().nth(next).unwrap());
                  next += columns;
            }
            start += 1;
      }
      decoded
  }
  