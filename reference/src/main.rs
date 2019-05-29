fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //change(&s); //Just as variables are immutable by default, so are     references. We’re not allowed to modify something we have a reference to.
    
    change(&mut s1);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}
// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
