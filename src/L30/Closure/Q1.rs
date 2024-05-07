/* Make it work with least amount of changes*/
fn main() {
    let color = String::from("green");

    let print = {
        let mut color = color.clone();
        move || println!("`color`: {}", color)
    };

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    let _reborrow = &color;

    println!("{}",color);
}