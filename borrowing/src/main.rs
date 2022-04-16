fn main() {
    let s = String::from("hi");
    let s1 = borrow(&s);
    println!("{}", s1);
    println!("{}", s);
}

fn borrow(s: &String) -> &String {
    s
}
