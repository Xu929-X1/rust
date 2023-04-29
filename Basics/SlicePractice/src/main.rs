fn main() {
    let s = String::from("Sample String");
    // let chinese = String::from("样本");
    // first_word(&chinese);
    let word = first_word(&s);
    println!("{word}"); //6, which is the index of the space in the byte array
    let first = &s[..word]; //.. is exclusive, ..= is inclusive
    println!("{}", first);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //this convert to u8 actually, using UTF8 format
    //iterate the bytes:
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //note here, the char needs to be a single quote
            return i;
        }
    }
    //if no space matched, return the length, as it is pointing to the last character
    s.len()
}
