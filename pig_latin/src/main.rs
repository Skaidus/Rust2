fn main() {
    let result = to_pig_latin(String::from("rust
    "));
    println!("{}",
        match result {
            Some(value) => value,
            None => String::from("Empty string")
        }
    );

    
}


fn to_pig_latin(mut s : String) -> Option<String>{
    if s.len()>0{
        let first_char = s.remove(0);
        s.push_str(format!("-{}ay", 
        match first_char {
            'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => 'h',
            other => other
        }).as_str());
        Some(s)
    } else {
        None
    }
}
/*
exists first letter:
    gives string
    gets first letter
    if consonant
        pop first
        add -
        add first
        add ay
    else 
        add -hay
else
    None

gives option string
*/
