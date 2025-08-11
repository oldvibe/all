pub fn reverse_it(v: i32) -> String {
    let mut res: String = String::new();
    let mut ress: String = String::new();

    let num = v.to_string();
    let mut negative = false;
    for el in num.chars() {
        // println!("{}", el);
        if el == '-' {
            negative = true;
            continue;
        }
        res.push(el);
    }
    for c in res.chars().rev() {
        ress.push(c);
    }
    //println!("ress {}", ress);
    if negative{
        return "-".to_string() + &ress + &res;
    }else{
        return ress + &res;
    }
}
