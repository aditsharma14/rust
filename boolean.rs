fn main() {
    let is_male = true;
    let is_above18 = true;
    if is_male {
        println!("You are a male");
    } else {
        println!("You are not a male");
    }
    if is_male && is_above18 {
        println!("You are above the legal age");
    }
}
