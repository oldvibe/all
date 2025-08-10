pub fn negative_spell(n: i64) -> String {
    if n > 0 {
        return "error: positive number".to_string();
    }
    if n == 0 {
        return "zero".to_string();
    }
    let abs_n = -n;
    "minus ".to_string() + &number_to_words(abs_n)
}

fn number_to_words(n: i64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20..=99 => {
            let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
            let t = tens[(n / 10) as usize];
            let u = n % 10;
            if u == 0 {
                t.to_string()
            } else {
                format!("{}-{}", t, number_to_words(u))
            }
        }
        100..=999 => {
            let h = n / 100;
            let rest = n % 100;
            if rest == 0 {
                format!("{} hundred", number_to_words(h))
            } else {
                format!("{} hundred {}", number_to_words(h), number_to_words(rest))
            }
        }
        1_000..=999_999 => {
            let th = n / 1000;
            let rest = n % 1000;
            if rest == 0 {
                format!("{} thousand", number_to_words(th))
            } else {
                format!("{} thousand {}", number_to_words(th), number_to_words(rest))
            }
        }
        1_000_000 => "one million".to_string(),
        _ => "out of range".to_string(),
    }
}
