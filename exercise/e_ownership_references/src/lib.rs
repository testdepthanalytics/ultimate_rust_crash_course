pub fn inspect(s: &String){
    let state = if s.ends_with("s"){
        "does"
    } else {
        "does not"
    };
    println!("{} {} end with s", s, state);
}

pub fn change(s: &mut String){
    if !s.ends_with("s") {
        s.push_str("s")
    }
}

pub fn eat(s: String) -> bool{
    return s.starts_with("b") && s.contains("a");
}

pub fn bedazzle(s: &mut String){
    *s = "sparkly".to_string()
}