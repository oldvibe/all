impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // ABO compatibility
        let abo_ok = match self.antigen {
            Antigen::O => matches!(other.antigen, Antigen::O),
            Antigen::A => matches!(other.antigen, Antigen::O | Antigen::A),
            Antigen::B => matches!(other.antigen, Antigen::O | Antigen::B),
            Antigen::AB => true,
        };

        // Rh compatibility
        let rh_ok = match self.rh_factor {
            RhFactor::Negative => matches!(other.rh_factor, RhFactor::Negative),
            RhFactor::Positive => true,
        };

        abo_ok && rh_ok
    }

    fn all_types() -> Vec<BloodType> {
        let antigens = [Antigen::O, Antigen::A, Antigen::B, Antigen::AB];
        let rhs = [RhFactor::Negative, RhFactor::Positive];
        let mut result = vec![];
        for a in &antigens {
            for r in &rhs {
                result.push(BloodType {
                    antigen: a.clone(),
                    rh_factor: r.clone(),
                });
            }
        }
        result
    }

    pub fn donors(&self) -> Vec<Self> {
        Self::all_types()
            .into_iter()
            .filter(|bt| self.can_receive_from(bt))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        Self::all_types()
            .into_iter()
            .filter(|bt| bt.can_receive_from(self))
            .collect()
    }
}


/////////////////////////////////////////////////////////////////////////////////////////////

```rs
const UNITS: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const GROUP: [&str; 3] = ["", "thousand", "million"];

pub fn negative_spell(n: i64) -> String {
    if n > 0 {
        return "error: positive number".to_string();
    }

    let mut res = String::new();
    let mut nbr = -n;
    let mut group = 0;

    if nbr == 0 {
        return "zero".to_string();
    }

    while nbr > 0 {
        let mut reminder = nbr % 1000;
        let mut temp_str: Vec<String> = Vec::new();

        if reminder >= 100 {
            temp_str.push(format!("{} hundred", UNITS[(reminder / 100) as usize]));
            reminder %= 100;
        }

        if reminder >= 20 {
            temp_str.push(TENS[(reminder / 10) as usize].to_string());
            reminder %= 10;
        }
        if reminder > 0 {
            temp_str.push(format!("{}", UNITS[reminder as usize]));
        }
        if group > 0 {
            temp_str.push(GROUP[group].to_string());
        }
        res = temp_str.join(" ") + " " + &res;
        res = res.trim().to_string();

        nbr /= 1000;
        group += 1;
    }

    format!("minus {}", res.trim())
}
```
//////////////////////////////////////////////////////////////

use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
   let mut res : HashMap<String, u32> = HashMap::new();
   for word in words.split_whitespace() {
      let cleaned : String = word.chars().filter(|c| c.is_alphanumeric() || *c == '\'').collect();
      let cleaned = cleaned.trim_matches('\'').to_lowercase();

      if !cleaned.is_empty() {
            *res.entry(cleaned).or_insert(0) += 1;
      }
   }
   res
}


//////////////////////////////////////
pub enum AccessLevel {
      Guest,
      Normal,
      Admin
}

pub struct User {
      name: String,
      acess_level: AccessLevel
}

impl User {
  pub fn new(name: String, level: AccessLevel) -> User {
      return User {
            name : name,
            acess_level : level
      }

  }
  pub fn send_name(&self) -> Option<&str> {
      match self.acess_level {
            AccessLevel::Guest => None,
            _=> Some(&self.name)
      }

  }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
      match user.send_name() {
            Some(name) => (true, name),
            None => (false, "ERROR: User is guest")
      }

}
/////////////////////////////////////////////////////////////////


use std::env;

fn rpn(input: &str) {
    let mut stack = Vec::<i64>::new();

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i64>() {
            stack.push(num);
        } else {
            let (b, a) = match (stack.pop(), stack.pop()) {
                (Some(b), Some(a)) => (b, a),
                _ => {
                    println!("Error");
                    return;
                }
            };

            let res = match token {
                "+" => a.checked_add(b),
                "-" => a.checked_sub(b),
                "*" => a.checked_mul(b),
                "/" => if b != 0 { a.checked_div(b) } else { None },
                "%" => if b != 0 { a.checked_rem(b) } else { None },
                _ => None,
            };

            match res {
                Some(val) => stack.push(val),
                None => {
                    println!("Error");
                    return;
                }
            }
        }
    }

    if stack.len() == 1 {
        println!("{}", stack[0]);
    } else {
        println!("Error");
    }
}
//////////////////////////////////////////////////////////////////////////////

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
  //////////////////////////////////////////////////////////////////////////

  #[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
            if rank > 7 || rank < 0 || file < 0 ||file > 7 {
                  return None;
            }
            Some(ChessPosition{
                  rank: rank,
                  file: file,
            })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
            Queen{
                  position: ChessPosition::new(position.rank, position.file).unwrap()
            }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
            self.position.rank == other.position.rank || self.position.file == other.position.file ||
            ((self.position.rank - other.position.rank) as f64).abs() == ((self.position.file - other.position.file) as f64).abs()  
    }
}