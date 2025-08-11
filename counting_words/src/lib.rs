use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();

    let mut strr: String = String::new();

    for el in words.chars() {
        if el.is_alphanumeric() || el.is_whitespace() || el == '\'' {
            strr.push(el);
        }
    }
    let strr = strr.replace("' ", " ").replace(" '", " ");
    let after = strr.split_whitespace();
    for el in after{
        *res.entry(el.to_lowercase()).or_insert(0) += 1;
    }
    res
}
