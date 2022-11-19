fn print(some_string: &String) {
    println!("{}", some_string);
}

fn change(some_string: &mut String) {
    some_string.push_str(", barbos");
}

fn change_print_delete(mut some_string: String) {
    some_string.push_str(", we smoke papiros");
    println!("{}", some_string);
    //some_string.clear();
}

fn main() {
    let mut s = String::from("hello");

    print(&s);

    change(&mut s);

    change_print_delete(s);
}