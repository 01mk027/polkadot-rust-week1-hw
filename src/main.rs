fn main() {
    let string1:String = String::from("Hello");
    let string2:String = String::from("World");
    let result:String = concatenate_strings(&string1, &string2);
    println!("{}", result);
}

fn concatenate_strings(slice1:&String, slice2:&String) -> String {
    let mut string1:String = slice1.clone();
    let mut string2:String = slice2.clone();
    let concatenated_slices = format!("{} {}", slice1, slice2);
    return String::from(concatenated_slices);
}
