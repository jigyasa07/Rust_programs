fn main() {
    let mut s =String::from("cbcdee abcdf");

    let r = first_word(&s);

    println!("{}",r);

    s.clear();// this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let slice1 = &s[..2];  //from starting
 
    let slice2 = &s[3..]; // till the last element

    let slice = &s[..]; //entire string
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();  //we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


