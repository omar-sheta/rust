fn main(){
    let mut s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(&s2);
    // q: explain mixing of immutable and mutable references
    // a: you can have one or the other, but not both at the same time
    println!("{}", word);
}


fn first_word(s : &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}